export type Identifier = { kind: 'named'; name: string } | { kind: 'indexed'; index: number };

export type Param =
  | { kind: 'simple'; identifier: Identifier }
  | { kind: 'default'; identifier: Identifier; word: Node[]; emptyAsUnset: boolean }
  | { kind: 'alt'; identifier: Identifier; word: Node[]; emptyAsUnset: boolean }
  | { kind: 'error'; identifier: Identifier; word: Node[]; emptyAsUnset: boolean }
  | { kind: 'length'; identifier: Identifier }
  | { kind: 'count' }
  | { kind: 'ref'; identifier: Identifier }
  | { kind: 'substring'; identifier: Identifier; offset: number; length: number | null }
  | { kind: 'case'; identifier: Identifier; change: CaseChange; all: boolean }
  | { kind: 'trimStart'; identifier: Identifier; pattern: Node[]; greedy: boolean }
  | { kind: 'trimEnd'; identifier: Identifier; pattern: Node[]; greedy: boolean }
  | {
      kind: 'replace';
      identifier: Identifier;
      pattern: Node[];
      replacement: Node[];
      all: boolean;
    };

export type CaseChange = 'upper' | 'lower' | 'reverse';

export type Node = { kind: 'text'; value: string } | { kind: 'param'; param: Param; index: number };
