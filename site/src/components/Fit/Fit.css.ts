import { style, styleVariants } from '@vanilla-extract/css';
import { colors } from '@/theme/colors.css';
import { vars } from '@/theme/theme-vars';

export const spectrum = style({
  display: 'grid',
  gridTemplateColumns: '1fr 1.25fr 1fr',
  marginTop: 48,
  overflow: 'hidden',
  border: `1px solid ${colors.line}`,
  borderRadius: 18,
  '@media': { '(max-width: 960px)': { gridTemplateColumns: 'minmax(0, 1fr)' } },
});

export const spot = style({
  display: 'flex',
  flexDirection: 'column',
  gap: 14,
  minWidth: 0,
  padding: 28,
  selectors: { '& + &': { borderLeft: `1px solid ${colors.line}` } },
  '@media': {
    '(max-width: 960px)': {
      selectors: { '& + &': { borderLeft: 0, borderTop: `1px solid ${colors.line}` } },
    },
  },
});

export const ours = style({ background: colors.bambooSoft });

export const name = style({
  fontFamily: vars.fontFamilyMonospace,
  fontSize: 16,
  fontWeight: 500,
  selectors: { [`${ours} &`]: { color: colors.bamboo } },
});

export const scale = style({
  display: 'flex',
  alignItems: 'center',
  gap: 8,
  fontFamily: vars.fontFamilyMonospace,
  fontSize: 12,
  color: colors.inkSoft,
});

export const track = style({
  position: 'relative',
  flex: 1,
  height: 6,
  overflow: 'hidden',
  borderRadius: 3,
  background: colors.line,
  '::after': {
    content: '""',
    position: 'absolute',
    inset: '0 auto 0 0',
    borderRadius: 3,
    background: colors.ink,
  },
  selectors: { [`${ours} &::after`]: { background: colors.bamboo } },
});

// How much of the shell's expansion each tool covers, roughly.
export const reach = styleVariants({
  envsubst: { '::after': { width: '8%' } },
  xpanda: { '::after': { width: '62%' } },
  eval: { '::after': { width: '100%' } },
});

export const body = style({ fontSize: 15, color: colors.inkSoft });

export const danger = style({ fontWeight: 600, color: colors.danger });

export const example = style({
  padding: '10px 12px',
  overflowX: 'auto',
  border: `1px solid ${colors.line}`,
  borderRadius: 8,
  background: colors.paper,
  fontFamily: vars.fontFamilyMonospace,
  fontSize: 13,
  whiteSpace: 'nowrap',
});

export const good = style({ color: colors.bamboo });

export const bad = style({ color: colors.danger });

export const verdict = style({ marginTop: 'auto', fontSize: 13, fontWeight: 600 });
