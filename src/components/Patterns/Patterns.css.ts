import { style } from '@vanilla-extract/css';
import { colors } from '@/theme/colors.css';
import { vars } from '@/theme/theme-vars';

export const tester = style({
  display: 'flex',
  flexWrap: 'wrap',
  alignItems: 'center',
  gap: '14px 22px',
  marginTop: 36,
  padding: '16px 18px',
  border: `1px solid ${colors.line}`,
  borderRadius: 14,
  background: colors.paperRaised,
});

export const valueLabel = style({
  display: 'flex',
  alignItems: 'center',
  gap: 8,
  fontFamily: vars.fontFamilyMonospace,
  fontSize: 14,
});

export const valueInput = style({
  width: '14em',
  maxWidth: '100%',
  padding: '7px 10px',
  border: `1px solid ${colors.line}`,
  borderRadius: 8,
  background: colors.paper,
  fontFamily: vars.fontFamilyMonospace,
  fontSize: 14,
  color: colors.ink,
  ':disabled': { opacity: 0.5 },
});

export const option = style({
  display: 'flex',
  alignItems: 'center',
  gap: 6,
  fontSize: 13,
  color: colors.inkSoft,
});

export const checkbox = style({ accentColor: colors.bamboo });

export const groups = style({
  display: 'grid',
  gridTemplateColumns: 'repeat(2, minmax(0, 1fr))',
  gap: '28px 40px',
  marginTop: 28,
  '@media': { '(max-width: 960px)': { gridTemplateColumns: 'minmax(0, 1fr)' } },
});

export const groupTitle = style({
  display: 'flex',
  justifyContent: 'space-between',
  paddingBottom: 8,
  borderBottom: `1.5px solid ${colors.ink}`,
  fontFamily: vars.fontFamily,
  fontSize: 14,
  fontWeight: 600,
});

export const groupCount = style({
  fontFamily: vars.fontFamilyMonospace,
  fontSize: 12,
  fontWeight: 400,
  color: colors.inkSoft,
});

export const row = style({
  display: 'grid',
  gridTemplateColumns: '11.5em minmax(0, 1fr) auto',
  alignItems: 'baseline',
  gap: 12,
  padding: '9px 0',
  borderBottom: `1px solid ${colors.line}`,
  fontSize: 14,
  '@media': { '(max-width: 640px)': { gridTemplateColumns: 'minmax(0, 1fr) auto' } },
});

export const pattern = style({ fontSize: 13, whiteSpace: 'nowrap' });

export const description = style({
  minWidth: 0,
  color: colors.inkSoft,
  '@media': {
    '(max-width: 640px)': { gridColumn: '1 / -1', gridRow: 2, fontSize: 13 },
  },
});

export const result = style({
  maxWidth: '12em',
  overflow: 'hidden',
  fontFamily: vars.fontFamilyMonospace,
  fontSize: 13,
  textAlign: 'right',
  textOverflow: 'ellipsis',
  whiteSpace: 'pre',
  color: colors.bamboo,
});

export const resultEmpty = style({ color: colors.inkSoft, opacity: 0.7 });

export const resultError = style({ color: colors.danger });
