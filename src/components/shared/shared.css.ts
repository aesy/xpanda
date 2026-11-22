import { style } from '@vanilla-extract/css';
import { colors } from '@/theme/colors.css';
import { vars } from '@/theme/theme-vars';

export const CONTAINER = 1440;

export const page = style({
  overflowX: 'clip',
  minHeight: '100vh',
  background: colors.paper,
  color: colors.ink,
});

// Doubled so it beats Mantine's own Container padding.
export const wrap = style({
  width: '100%',
  boxSizing: 'border-box',
  selectors: {
    '&&': {
      paddingLeft: 'max(24px, env(safe-area-inset-left, 0px))',
      paddingRight: 'max(24px, env(safe-area-inset-right, 0px))',
    },
  },
  '@media': {
    '(max-width: 640px)': {
      selectors: {
        '&&': {
          paddingLeft: 'max(16px, env(safe-area-inset-left, 0px))',
          paddingRight: 'max(16px, env(safe-area-inset-right, 0px))',
        },
      },
    },
  },
});

export const section = style({
  borderTop: `1px solid ${colors.line}`,
  paddingBlock: 80,
  '@media': { '(max-width: 640px)': { paddingBlock: 56 } },
});

export const heading = style({
  maxWidth: '18em',
  color: colors.bamboo,
  fontFamily: vars.fontFamilyHeadings,
  fontStyle: 'italic',
  fontWeight: 600,
  fontSize: 'clamp(2rem, 4.2vw, 3rem)',
  lineHeight: 1.08,
});

export const intro = style({ maxWidth: '40em', marginTop: 14, color: colors.inkSoft });

export const inlineCode = style({ fontSize: '0.9em', color: colors.ink });
