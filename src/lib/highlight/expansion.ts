export type SyntaxKind = 'plain' | 'variable' | 'operator' | 'word' | 'number' | 'comment';

export type SyntaxToken = { kind: SyntaxKind; text: string };

const SIMPLE = /^\$(?:[A-Za-z_][A-Za-z0-9_]*|[0-9]+)/;
const HEAD =
  /^([#!]?)([A-Za-z_][A-Za-z0-9_]*|[0-9]+)?(:-|:\+|:\?|\^\^|,,|~~|##|%%|\/\/|[-+?^,~#%/:])?([\s\S]*)$/;

function push(tokens: SyntaxToken[], kind: SyntaxKind, text: string) {
  if (text) {
    tokens.push({ kind, text });
  }
}

function closingBrace(source: string, start: number): number {
  let depth = 0;

  for (let i = start; i < source.length; i++) {
    if (source.startsWith('${', i)) {
      depth++;
      i++;
    } else if (source[i] === '}' && --depth === 0) {
      return i;
    }
  }

  return source.length;
}

// Lenient on purpose: it colors what the user is still typing, so it never throws.
function expansion(source: string, plain: SyntaxKind, tokens: SyntaxToken[]) {
  let text = '';
  let i = 0;

  const flush = () => {
    push(tokens, plain, text);
    text = '';
  };

  while (i < source.length) {
    if (source.startsWith('$$', i)) {
      text += '$$';
      i += 2;
    } else if (source.startsWith('${', i)) {
      flush();
      const end = closingBrace(source, i);
      push(tokens, 'operator', '${');
      inner(source.slice(i + 2, end), tokens);

      if (end < source.length) {
        push(tokens, 'operator', '}');
      }

      i = end + 1;
    } else {
      const simple = source[i] === '$' ? SIMPLE.exec(source.slice(i)) : null;

      if (simple) {
        flush();
        push(tokens, 'variable', simple[0]);
        i += simple[0].length;
      } else {
        text += source[i++];
      }
    }
  }

  flush();
}

function inner(source: string, tokens: SyntaxToken[]) {
  const [, prefix = '', name = '', operator = '', rest = ''] = HEAD.exec(source) ?? [];
  push(tokens, 'operator', prefix);
  push(tokens, 'variable', name);
  push(tokens, 'operator', operator);

  if (operator === ':') {
    for (const part of rest.match(/-?\d+|[^\d-]+|-/g) ?? []) {
      push(tokens, /\d/.test(part) ? 'number' : 'operator', part);
    }
  } else if (operator === '/' || operator === '//') {
    const separator = rest.indexOf('/');

    if (separator === -1) {
      expansion(rest, 'word', tokens);
    } else {
      expansion(rest.slice(0, separator), 'word', tokens);
      push(tokens, 'operator', '/');
      expansion(rest.slice(separator + 1), 'word', tokens);
    }
  } else {
    expansion(rest, 'word', tokens);
  }
}

export function highlightExpansion(source: string): SyntaxToken[] {
  const tokens: SyntaxToken[] = [];
  expansion(source, 'plain', tokens);

  return tokens;
}

// Lines starting with `#` are comments in the demo's config template,
// but xpanda still expands the params inside them.
export function highlightTemplate(source: string): SyntaxToken[] {
  const tokens: SyntaxToken[] = [];

  source.split('\n').forEach((line, index) => {
    if (index > 0) {
      push(tokens, 'plain', '\n');
    }

    expansion(line, line.trimStart().startsWith('#') ? 'comment' : 'plain', tokens);
  });

  return tokens;
}

export function highlightEnvFile(source: string): SyntaxToken[] {
  const tokens: SyntaxToken[] = [];

  source.split('\n').forEach((line, index) => {
    if (index > 0) {
      push(tokens, 'plain', '\n');
    }

    const assignment = /^(\s*)([^=#\s][^=]*)(=)(.*)$/.exec(line);

    if (line.trimStart().startsWith('#')) {
      push(tokens, 'comment', line);
    } else if (assignment) {
      push(tokens, 'plain', assignment[1]);
      push(tokens, 'variable', assignment[2]);
      push(tokens, 'operator', assignment[3]);
      push(tokens, 'word', assignment[4]);
    } else {
      push(tokens, 'plain', line);
    }
  });

  return tokens;
}
