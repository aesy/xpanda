import { Glob } from '@/lib/expand/glob';

describe('Glob', () => {
  it('matches classes, ranges and escapes', () => {
    expect(new Glob('[a-c]x').matches('bx')).toBe(true);
    expect(new Glob('[!a-c]x').matches('bx')).toBe(false);
    expect(new Glob('a\\*').matches('a*')).toBe(true);
    expect(new Glob('a\\*').matches('ab')).toBe(false);
    expect(new Glob('[a').matches('[a')).toBe(true);
  });

  it('trims the shortest or longest prefix and suffix', () => {
    const glob = new Glob('*.');
    expect(glob.trimStart('a.b.c', false)).toBe('b.c');
    expect(glob.trimStart('a.b.c', true)).toBe('c');
    expect(new Glob('.*').trimEnd('a.b.c', false)).toBe('a.b');
    expect(new Glob('.*').trimEnd('a.b.c', true)).toBe('a');
  });

  it('replaces the first or every match', () => {
    expect(new Glob('o').replace('foo', '0', false)).toBe('f0o');
    expect(new Glob('o').replace('foo', '0', true)).toBe('f00');
    expect(new Glob('').replace('foo', 'x', true)).toBe('foo');
  });
});
