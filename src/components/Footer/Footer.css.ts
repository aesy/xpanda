import { style } from '@vanilla-extract/css';
import { colors } from '@/theme/colors.css';

export const footer = style({ borderTop: `1px solid ${colors.line}` });

export const row = style({
  display: 'flex',
  flexWrap: 'wrap',
  alignItems: 'center',
  justifyContent: 'space-between',
  gap: 20,
  paddingBlock: 36,
  fontSize: 13,
  color: colors.inkSoft,
});

export const logo = style({ display: 'block', height: 28, width: 'auto', color: colors.ink });

export const links = style({ display: 'flex', flexWrap: 'wrap', gap: 20 });

export const link = style({ ':hover': { color: colors.ink } });
