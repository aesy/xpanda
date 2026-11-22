export type ErrorKind =
  | { kind: 'expected'; expected: string; found: string }
  | { kind: 'invalidParamToken'; token: string }
  | { kind: 'invalidParamEof' }
  | { kind: 'invalidNumber'; value: string }
  | { kind: 'unset'; name: string }
  | { kind: 'unsetOrEmpty'; name: string }
  | { kind: 'userMessage'; message: string }
  | { kind: 'trailingBackslash' };

// Worded like the crate's ErrorKind Display, since the demo stands in for its output.
function describe(error: ErrorKind): string {
  switch (error.kind) {
    case 'expected':
      return `Expected ${error.expected}, found ${error.found}`;
    case 'invalidParamToken':
      return `Invalid param, unexpected token ${error.token}`;
    case 'invalidParamEof':
      return 'Invalid param, unexpected EOF';
    case 'invalidNumber':
      return `Invalid number: "${error.value}"`;
    case 'unset':
      return `'${error.name}' is unset`;
    case 'unsetOrEmpty':
      return `'${error.name}' is unset or empty`;
    case 'userMessage':
      return error.message;
    case 'trailingBackslash':
      return 'Trailing backslash in pattern';
  }
}

export class ExpandError extends Error {
  readonly error: ErrorKind;
  index: number | null;

  constructor(error: ErrorKind, index: number | null = null) {
    super(describe(error));
    this.error = error;
    this.index = index;
  }
}
