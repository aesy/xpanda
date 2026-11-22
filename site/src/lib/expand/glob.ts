import { ExpandError } from '@/lib/expand/error';

type Enclosed = { kind: 'char'; char: string } | { kind: 'range'; from: string; to: string };

type Token =
  | { kind: 'anyString' }
  | { kind: 'anyChar' }
  | { kind: 'exact'; char: string }
  | { kind: 'anyOf'; variants: Enclosed[]; negated: boolean };

function compile(pattern: string): Token[] {
  const chars = [...pattern];
  const tokens: Token[] = [];
  let i = 0;

  while (i < chars.length) {
    const char = chars[i++];

    if (char === '*') {
      if (tokens.at(-1)?.kind !== 'anyString') {
        tokens.push({ kind: 'anyString' });
      }
    } else if (char === '?') {
      tokens.push({ kind: 'anyChar' });
    } else if (char === '\\') {
      if (i >= chars.length) {
        throw new ExpandError({ kind: 'trailingBackslash' });
      }

      tokens.push({ kind: 'exact', char: chars[i++] });
    } else if (char === '[' && chars.slice(i).includes(']')) {
      const negated = chars[i] === '!' || chars[i] === '^';

      if (negated) {
        i++;
      }

      const variants: Enclosed[] = [];

      while (i < chars.length && chars[i] !== ']') {
        const from = chars[i++];

        if (chars[i] !== '-') {
          variants.push({ kind: 'char', char: from });
          continue;
        }

        i++;
        const to = chars[i];

        if (to === undefined || to === ']') {
          variants.push({ kind: 'char', char: from }, { kind: 'char', char: '-' });
        } else {
          variants.push({ kind: 'range', from, to });
          i++;
        }
      }

      i++;
      tokens.push({ kind: 'anyOf', variants, negated });
    } else {
      tokens.push({ kind: 'exact', char });
    }
  }

  return tokens;
}

function matchesOne(token: Token, char: string | undefined): boolean {
  if (char === undefined) {
    return false;
  }

  switch (token.kind) {
    case 'anyChar':
      return true;
    case 'exact':
      return token.char === char;
    case 'anyOf': {
      const hit = token.variants.some(variant =>
        variant.kind === 'char' ? variant.char === char : variant.from <= char && char <= variant.to
      );

      return hit !== token.negated;
    }
    case 'anyString':
      return false;
  }
}

// The lengths of every prefix of `input` that the pattern consumes in full.
function matchLengths(tokens: Token[], input: string[]): number[] {
  let positions = [0];

  for (const token of tokens) {
    if (positions.length === 0) {
      break;
    }

    if (token.kind === 'anyString') {
      const first = positions[0];
      positions = Array.from({ length: input.length - first + 1 }, (_, k) => first + k);
    } else {
      positions = positions.filter(p => matchesOne(token, input[p])).map(p => p + 1);
    }
  }

  return positions;
}

function pick(lengths: number[], greedy: boolean): number | undefined {
  if (lengths.length === 0) {
    return undefined;
  }

  return greedy ? Math.max(...lengths) : Math.min(...lengths);
}

export class Glob {
  private readonly tokens: Token[];

  constructor(pattern: string) {
    this.tokens = compile(pattern);
  }

  matches(input: string): boolean {
    const chars = [...input];

    return matchLengths(this.tokens, chars).includes(chars.length);
  }

  trimStart(input: string, greedy: boolean): string {
    const chars = [...input];
    const length = pick(matchLengths(this.tokens, chars), greedy);

    return length === undefined ? input : chars.slice(length).join('');
  }

  trimEnd(input: string, greedy: boolean): string {
    const chars = [...input];
    const candidates: number[] = [];

    for (let start = 0; start <= chars.length; start++) {
      const suffix = chars.slice(start);

      if (matchLengths(this.tokens, suffix).includes(suffix.length)) {
        candidates.push(suffix.length);
      }
    }

    const length = pick(candidates, greedy);

    return length === undefined ? input : chars.slice(0, chars.length - length).join('');
  }

  replace(input: string, replacement: string, all: boolean): string {
    if (input === '' && this.tokens.length > 0 && this.matches(input)) {
      return replacement;
    }

    const chars = [...input];
    let out = '';
    let i = 0;

    while (i < chars.length) {
      const length = pick(matchLengths(this.tokens, chars.slice(i)), true);

      if (length === undefined || length === 0) {
        out += chars[i++];
        continue;
      }

      out += replacement;
      i += length;

      if (!all) {
        return out + chars.slice(i).join('');
      }
    }

    return out;
  }
}
