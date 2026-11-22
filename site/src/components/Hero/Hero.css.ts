import { style } from '@vanilla-extract/css';
import { colors } from '@/theme/colors.css';
import { vars } from '@/theme/theme-vars';

export const grid = style({
  display: 'grid',
  gridTemplateColumns: 'minmax(0, 36rem) minmax(0, 1fr)',
  gap: 56,
  alignItems: 'start',
  paddingBlock: '72px 80px',
  '@media': {
    // Below this the demo gets too narrow for its two panes, so the text goes on top.
    '(max-width: 1280px)': {
      gridTemplateColumns: 'minmax(0, 1fr)',
      gap: 40,
      paddingBlock: '48px 56px',
    },
  },
});

export const title = style({
  fontFamily: vars.fontFamilyHeadings,
  fontStyle: 'italic',
  fontWeight: 600,
  fontSize: 'clamp(2.5rem, 6vw, 4.25rem)',
  lineHeight: 1.02,
  letterSpacing: '-0.01em',
});

export const titleLine = style({ display: 'block' });

export const titleToken = style({
  fontFamily: vars.fontFamilyMonospace,
  fontStyle: 'normal',
  fontWeight: 500,
  fontSize: '0.62em',
  letterSpacing: 0,
  verticalAlign: '0.06em',
});

export const lede = style({
  maxWidth: '34em',
  margin: '22px 0 28px',
  fontSize: 17,
  color: colors.inkSoft,
});

export const ctas = style({ display: 'flex', flexWrap: 'wrap', gap: 12 });

const button = style({
  display: 'inline-flex',
  alignItems: 'center',
  padding: '11px 18px',
  border: `1.5px solid ${colors.ink}`,
  borderRadius: 999,
  fontSize: 15,
  fontWeight: 600,
  textDecoration: 'none',
  ':hover': { transform: 'translateY(-1px)' },
  '@media': { '(prefers-reduced-motion: reduce)': { ':hover': { transform: 'none' } } },
});

export const solidButton = style([button, { background: colors.ink, color: colors.paper }]);

export const ghostButton = style([button, { background: 'transparent', color: colors.ink }]);

export const installs = style({ display: 'grid', gap: 10, marginTop: 32 });

export const facts = style({
  display: 'flex',
  flexWrap: 'wrap',
  gap: 18,
  marginTop: 18,
  fontSize: 13,
  color: colors.inkSoft,
});
