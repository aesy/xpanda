import type { CaseChange, Identifier, Node, Param } from '@/lib/expand/ast';
import { ExpandError } from '@/lib/expand/error';
import { Glob } from '@/lib/expand/glob';

export type Vars = {
  named: ReadonlyMap<string, string>;
  positional: readonly string[];
  noUnset: boolean;
};

function identifierName(identifier: Identifier): string {
  return identifier.kind === 'named' ? identifier.name : String(identifier.index);
}

function changeCase(char: string, change: CaseChange): string {
  switch (change) {
    case 'upper':
      return char.toUpperCase();
    case 'lower':
      return char.toLowerCase();
    case 'reverse': {
      const upper = char.toUpperCase();

      return char === upper ? char.toLowerCase() : upper;
    }
  }
}

export class Evaluator {
  private readonly vars: Vars;

  constructor(vars: Vars) {
    this.vars = vars;
  }

  nodes(nodes: Node[]): string {
    return nodes.map(node => (node.kind === 'text' ? node.value : this.paramAt(node))).join('');
  }

  private paramAt(node: Extract<Node, { kind: 'param' }>): string {
    try {
      return this.param(node.param);
    } catch (error) {
      if (error instanceof ExpandError && error.index === null) {
        error.index = node.index;
      }

      throw error;
    }
  }

  private lookup(identifier: Identifier): string | undefined {
    if (identifier.kind === 'named') {
      return this.vars.named.get(identifier.name);
    }

    if (identifier.index === 0) {
      return this.vars.positional.join(' ');
    }

    return this.vars.positional[identifier.index - 1];
  }

  private lookupWithValue(identifier: Identifier, emptyAsUnset: boolean): string | undefined {
    const value = this.lookup(identifier);

    return emptyAsUnset && value === '' ? undefined : value;
  }

  private unsetError(identifier: Identifier, emptyAsUnset: boolean): ExpandError {
    const name = identifierName(identifier);

    return new ExpandError(emptyAsUnset ? { kind: 'unsetOrEmpty', name } : { kind: 'unset', name });
  }

  private simple(identifier: Identifier): string {
    const value = this.lookup(identifier);

    if (value !== undefined) {
      return value;
    }

    if (this.vars.noUnset) {
      throw this.unsetError(identifier, false);
    }

    return '';
  }

  private param(param: Param): string {
    switch (param.kind) {
      case 'simple':
        return this.simple(param.identifier);
      case 'default':
        return this.lookupWithValue(param.identifier, param.emptyAsUnset) ?? this.nodes(param.word);
      case 'alt':
        return this.lookupWithValue(param.identifier, param.emptyAsUnset) === undefined
          ? ''
          : this.nodes(param.word);
      case 'error': {
        const value = this.lookupWithValue(param.identifier, param.emptyAsUnset);

        if (value !== undefined) {
          return value;
        }

        if (param.word.length === 0) {
          throw this.unsetError(param.identifier, param.emptyAsUnset);
        }

        throw new ExpandError({ kind: 'userMessage', message: this.nodes(param.word) });
      }
      case 'length': {
        const value = this.lookup(param.identifier);

        if (value !== undefined) {
          return String([...value].length);
        }

        if (this.vars.noUnset) {
          throw this.unsetError(param.identifier, false);
        }

        return '0';
      }
      case 'count':
        return String(this.vars.positional.length);
      case 'ref': {
        const name = this.simple(param.identifier);
        const target: Identifier = /^\d+$/.test(name)
          ? { kind: 'indexed', index: Number(name) }
          : { kind: 'named', name };

        return this.simple(target);
      }
      case 'substring':
        return this.substring(this.simple(param.identifier), param.offset, param.length);
      case 'case': {
        const chars = [...this.simple(param.identifier)];

        if (param.all) {
          return chars.map(char => changeCase(char, param.change)).join('');
        }

        return chars.length === 0
          ? ''
          : changeCase(chars[0], param.change) + chars.slice(1).join('');
      }
      case 'trimStart':
        return new Glob(this.nodes(param.pattern)).trimStart(
          this.simple(param.identifier),
          param.greedy
        );
      case 'trimEnd':
        return new Glob(this.nodes(param.pattern)).trimEnd(
          this.simple(param.identifier),
          param.greedy
        );
      case 'replace': {
        const value = this.simple(param.identifier);
        const glob = new Glob(this.nodes(param.pattern));

        return glob.replace(value, this.nodes(param.replacement), param.all);
      }
    }
  }

  private substring(value: string, offset: number, length: number | null): string {
    const chars = [...value];
    const len = chars.length;
    let start: number;

    if (offset < 0) {
      // Bash yields nothing when a negative offset reaches past the start.
      if (-offset > len) {
        return '';
      }

      start = len + offset;
    } else {
      start = Math.min(offset, len);
    }

    let end = len;

    if (length !== null) {
      const index = length < 0 ? Math.max(len + length, 0) : start + length;
      end = Math.max(Math.min(index, len), start);
    }

    return chars.slice(start, end).join('');
  }
}
