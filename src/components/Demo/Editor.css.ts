import { style } from '@vanilla-extract/css';
import { panel } from '@/theme/colors.css';
import { vars } from '@/theme/theme-vars';

export const editor = style({ display: 'grid', flex: 1 });

// The textarea sits over the colored copy of its text; both must lay out identically.
const layer = style({
  gridArea: '1 / 1',
  boxSizing: 'border-box',
  width: '100%',
  minHeight: 196,
  margin: 0,
  padding: '8px 18px 16px',
  border: 0,
  fontFamily: vars.fontFamilyMonospace,
  fontSize: 13,
  lineHeight: 1.65,
  letterSpacing: 0,
  tabSize: 2,
  whiteSpace: 'pre-wrap',
  overflowWrap: 'anywhere',
  '@media': { '(max-width: 640px)': { minHeight: 150 } },
});

export const colored = style([layer, { color: panel.text, pointerEvents: 'none' }]);

export const input = style([
  layer,
  {
    overflow: 'hidden',
    resize: 'none',
    background: 'transparent',
    color: 'transparent',
    caretColor: panel.text,
    // The panel clips its overflow, so the ring goes inside the box; the global
    // focus style puts it outside, where only the top and bottom showed.
    selectors: {
      '&:focus-visible': { outline: `2px solid ${panel.accent}`, outlineOffset: -2 },
    },
    '::selection': { background: 'rgba(166, 211, 111, 0.3)', color: 'transparent' },
  },
]);
