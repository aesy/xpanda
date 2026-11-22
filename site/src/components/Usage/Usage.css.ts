import { style } from '@vanilla-extract/css';
import { colors } from '@/theme/colors.css';
import { vars } from '@/theme/theme-vars';

export const uses = style({
  display: 'grid',
  gridTemplateColumns: 'repeat(2, minmax(0, 1fr))',
  gap: 28,
  marginTop: 44,
  '@media': { '(max-width: 960px)': { gridTemplateColumns: 'minmax(0, 1fr)' } },
});

export const use = style({ minWidth: 0 });

export const title = style({
  display: 'flex',
  alignItems: 'baseline',
  gap: 10,
  fontFamily: vars.fontFamily,
  fontSize: 20,
  fontWeight: 600,
});

export const crate = style({
  fontFamily: vars.fontFamilyMonospace,
  fontSize: 12,
  fontWeight: 400,
  color: colors.inkSoft,
});

export const body = style({ margin: '8px 0 16px', fontSize: 15, color: colors.inkSoft });

export const flags = style({ display: 'grid', gap: 6, marginTop: 16, fontSize: 14 });

export const flag = style({
  display: 'grid',
  gridTemplateColumns: '9.5em minmax(0, 1fr)',
  gap: 12,
});

export const flagName = style({ color: colors.bamboo });

export const flagText = style({ color: colors.inkSoft });
