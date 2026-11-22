import type { CaseChange, Identifier, Node, Param } from '@/lib/expand/ast';
import { ExpandError } from '@/lib/expand/error';

const NAME = /[A-Za-z_][A-Za-z0-9_]*|[0-9]+/y;
const NUMBER = /\s*(\(\s*-?\d+\s*\)|-?\d+)\s*/y;

const WORD_OPERATORS = [
  [':-', 'default', true],
  [':+', 'alt', true],
  [':?', 'error', true],
  ['-', 'default', false],
  ['+', 'alt', false],
  ['?', 'error', false],
] as const;

const CASE_OPERATORS: readonly (readonly [string, CaseChange, boolean])[] = [
  ['^^', 'upper', true],
  ['^', 'upper', false],
  [',,', 'lower', true],
  [',', 'lower', false],
  ['~~', 'reverse', true],
  ['~', 'reverse', false],
];

class Parser {
  private index = 0;
  private readonly source: string;

  constructor(source: string) {
    this.source = source;
  }

  parse(): Node[] {
    return this.nodes([]);
  }

  private peek(offset = 0): string | undefined {
    return this.source[this.index + offset];
  }

  private eat(text: string): boolean {
    if (this.source.startsWith(text, this.index)) {
      this.index += text.length;

      return true;
    }

    return false;
  }

  private nodes(stops: readonly string[]): Node[] {
    const nodes: Node[] = [];
    let text = '';

    const flush = () => {
      if (text) {
        nodes.push({ kind: 'text', value: text });
        text = '';
      }
    };

    while (this.index < this.source.length && !stops.includes(this.source[this.index])) {
      if (this.eat('$$')) {
        text += '$';
      } else if (this.peek() === '$') {
        flush();
        const index = this.index;
        nodes.push({ kind: 'param', param: this.param(), index });
      } else {
        text += this.source[this.index++];
      }
    }

    flush();

    return nodes;
  }

  private unexpected(): ExpandError {
    const token = this.peek();

    return token === undefined
      ? new ExpandError({ kind: 'invalidParamEof' }, this.index)
      : new ExpandError({ kind: 'invalidParamToken', token }, this.index);
  }

  private identifier(): Identifier {
    NAME.lastIndex = this.index;
    const match = NAME.exec(this.source);

    if (!match) {
      throw this.unexpected();
    }

    this.index += match[0].length;
    const name = match[0];

    return /^\d/.test(name) ? { kind: 'indexed', index: Number(name) } : { kind: 'named', name };
  }

  private close() {
    if (!this.eat('}')) {
      throw this.unexpected();
    }
  }

  private number(): number {
    NUMBER.lastIndex = this.index;
    const match = NUMBER.exec(this.source);

    if (!match) {
      const end = this.source.slice(this.index).search(/[:}]/);
      const value =
        end === -1
          ? this.source.slice(this.index)
          : this.source.slice(this.index, this.index + end);
      throw new ExpandError({ kind: 'invalidNumber', value }, this.index);
    }

    this.index += match[0].length;

    return Number(match[1].replace(/[()\s]/g, ''));
  }

  private param(): Param {
    this.index += 1;

    if (!this.eat('{')) {
      return { kind: 'simple', identifier: this.identifier() };
    }

    if (this.eat('#}')) {
      return { kind: 'count' };
    }

    if (this.eat('#')) {
      const identifier = this.identifier();
      this.close();

      return { kind: 'length', identifier };
    }

    if (this.eat('!')) {
      const identifier = this.identifier();
      this.close();

      return { kind: 'ref', identifier };
    }

    const identifier = this.identifier();

    if (this.eat('}')) {
      return { kind: 'simple', identifier };
    }

    for (const [operator, kind, emptyAsUnset] of WORD_OPERATORS) {
      if (this.eat(operator)) {
        const word = this.nodes(['}']);
        this.close();

        return { kind, identifier, word, emptyAsUnset };
      }
    }

    for (const [operator, change, all] of CASE_OPERATORS) {
      if (this.eat(operator)) {
        this.close();

        return { kind: 'case', identifier, change, all };
      }
    }

    for (const [operator, kind] of [
      ['#', 'trimStart'],
      ['%', 'trimEnd'],
    ] as const) {
      if (this.eat(operator)) {
        const greedy = this.eat(operator);
        const pattern = this.nodes(['}']);
        this.close();

        return { kind, identifier, pattern, greedy };
      }
    }

    if (this.eat('/')) {
      const all = this.eat('/');
      const pattern = this.nodes(['/', '}']);
      const replacement = this.eat('/') ? this.nodes(['}']) : [];
      this.close();

      return { kind: 'replace', identifier, pattern, replacement, all };
    }

    if (this.eat(':')) {
      const offset = this.number();
      const length = this.eat(':') ? this.number() : null;
      this.close();

      return { kind: 'substring', identifier, offset, length };
    }

    throw this.unexpected();
  }
}

export function parse(source: string): Node[] {
  return new Parser(source).parse();
}
