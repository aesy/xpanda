import { highlightEnvFile, highlightExpansion, highlightTemplate } from '@/lib/highlight/expansion';

const kinds = (tokens: { kind: string; text: string }[]) => tokens.map(t => `${t.kind}:${t.text}`);

describe('highlightExpansion', () => {
  it('splits a param into its parts', () => {
    expect(kinds(highlightExpansion('${PORT:-8080}'))).toEqual([
      'operator:${',
      'variable:PORT',
      'operator::-',
      'word:8080',
      'operator:}',
    ]);
  });

  it('colors substring numbers and replace separators', () => {
    expect(kinds(highlightExpansion('${VAR:0:7}'))).toEqual([
      'operator:${',
      'variable:VAR',
      'operator::',
      'number:0',
      'operator::',
      'number:7',
      'operator:}',
    ]);
    expect(kinds(highlightExpansion('${VAR//e/E}'))).toContain('operator:/');
  });

  it('keeps the text around simple and nested params', () => {
    expect(kinds(highlightExpansion('a $B ${C:-$D}'))).toEqual([
      'plain:a ',
      'variable:$B',
      'plain: ',
      'operator:${',
      'variable:C',
      'operator::-',
      'variable:$D',
      'operator:}',
    ]);
  });

  it('does not throw on an unfinished param', () => {
    expect(kinds(highlightExpansion('${VAR:-'))).toEqual([
      'operator:${',
      'variable:VAR',
      'operator::-',
    ]);
  });
});

describe('highlightTemplate', () => {
  it('dims comment lines but still colors their params', () => {
    expect(kinds(highlightTemplate('x\n# by ${USER}'))).toEqual([
      'plain:x',
      'plain:\n',
      'comment:# by ',
      'operator:${',
      'variable:USER',
      'operator:}',
    ]);
  });
});

describe('highlightEnvFile', () => {
  it('colors names, the separator and values', () => {
    expect(kinds(highlightEnvFile('HOST=a.org\n# note'))).toEqual([
      'variable:HOST',
      'operator:=',
      'word:a.org',
      'plain:\n',
      'comment:# note',
    ]);
  });
});
