import en from '@/locales/en.json';

describe('locales', () => {
  it('use plain ASCII punctuation', () => {
    expect(JSON.stringify(en)).not.toMatch(/[–—‘’“”−]/);
  });
});
