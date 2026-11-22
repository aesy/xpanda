import { expand } from '@/lib/expand/expand';

const run = (source: string, value: string | undefined, noUnset = false) =>
  expand(source, {
    named: new Map(value === undefined ? [] : [['VAR', value]]),
    positional: [],
    noUnset,
  });

const text = (source: string, value: string | undefined) => {
  const result = run(source, value);

  return result.ok ? result.text : `error: ${result.message}`;
};

// The examples table in docs/PATTERNS.md: VAR unset, VAR="" and VAR="example".
const EXAMPLES: [string, string, string, string][] = [
  ['$VAR', '', '', 'example'],
  ['${VAR}', '', '', 'example'],
  ['${VAR-default}', 'default', '', 'example'],
  ['${VAR:-default}', 'default', 'default', 'example'],
  ['${VAR+alternative}', '', 'alternative', 'alternative'],
  ['${VAR:+alternative}', '', '', 'alternative'],
  ['${VAR?message}', 'error: message', '', 'example'],
  ['${VAR:?message}', 'error: message', 'error: message', 'example'],
  ['${#VAR}', '0', '0', '7'],
  ['${!VAR}', '', '', ''],
  ['${VAR:2}', '', '', 'ample'],
  ['${VAR:2:4}', '', '', 'ampl'],
  ['${VAR^}', '', '', 'Example'],
  ['${VAR^^}', '', '', 'EXAMPLE'],
  ['${VAR,}', '', '', 'example'],
  ['${VAR,,}', '', '', 'example'],
  ['${VAR~}', '', '', 'Example'],
  ['${VAR~~}', '', '', 'EXAMPLE'],
  ['${VAR#*e}', '', '', 'xample'],
  ['${VAR##*e}', '', '', ''],
  ['${VAR%e*}', '', '', 'exampl'],
  ['${VAR%%e*}', '', '', ''],
  ['${VAR/e/E}', '', '', 'Example'],
  ['${VAR//e/E}', '', '', 'ExamplE'],
];

describe('expand', () => {
  it.each(EXAMPLES)('%s', (source, unset, empty, example) => {
    expect(text(source, undefined)).toBe(unset);
    expect(text(source, '')).toBe(empty);
    expect(text(source, 'example')).toBe(example);
  });

  it('fails on unset variables with no default when no-unset is on', () => {
    expect(run('a\n  ${VAR^}', undefined, true)).toEqual({
      ok: false,
      message: "'VAR' is unset",
      line: 2,
      column: 3,
    });
    expect(run('${VAR:-fine}', undefined, true)).toEqual({ ok: true, text: 'fine' });
  });

  it('names the variable when a required one is empty', () => {
    expect(run('${VAR:?}', '')).toMatchObject({ ok: false, message: "'VAR' is unset or empty" });
  });

  it('expands nested defaults', () => {
    expect(text('${VAR:-${OTHER:-deep}}', undefined)).toBe('deep');
  });

  it('takes the last characters for a negative offset', () => {
    expect(text('${VAR: -3}', 'example')).toBe('ple');
    expect(text('${VAR:(-3):2}', 'example')).toBe('pl');
    expect(text('${VAR: -9}', 'example')).toBe('');
  });

  it('keeps an escaped dollar sign', () => {
    expect(text('cost: $$5', undefined)).toBe('cost: $5');
  });

  it('reports a param it cannot parse', () => {
    expect(run('${VAR', 'x')).toMatchObject({
      ok: false,
      message: 'Invalid param, unexpected EOF',
    });
    expect(run('${VAR*}', 'x')).toMatchObject({
      ok: false,
      message: 'Invalid param, unexpected token *',
    });
    expect(run('${VAR:x}', 'x')).toMatchObject({ ok: false, message: 'Invalid number: "x"' });
  });
});
