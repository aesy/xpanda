import { globalStyle } from '@vanilla-extract/css';
import { colors } from '@/theme/colors.css';
import { vars } from '@/theme/theme-vars';

globalStyle('html', {
  scrollBehavior: 'smooth',
  scrollPaddingTop: 24,
  '@media': {
    '(prefers-reduced-motion: reduce)': { scrollBehavior: 'auto' },
  },
});

globalStyle('body', {
  margin: 0,
  fontFamily: vars.fontFamily,
  fontSize: 16,
  lineHeight: 1.6,
  color: colors.ink,
  background: colors.paper,
  WebkitFontSmoothing: 'antialiased',
});

globalStyle('h1, h2, h3, p, pre, dl, dd', { margin: 0 });

globalStyle('h1, h2, h3', { textWrap: 'balance' });

globalStyle('code, pre, textarea', { fontFamily: vars.fontFamilyMonospace });

globalStyle('a', { color: 'inherit' });

globalStyle('button', { fontFamily: 'inherit' });

globalStyle('a:focus-visible, button:focus-visible, input:focus-visible, textarea:focus-visible', {
  outline: `2px solid ${colors.bamboo}`,
  outlineOffset: 2,
});
