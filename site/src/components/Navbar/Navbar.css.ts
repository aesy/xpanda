import { style } from '@vanilla-extract/css';
import { colors } from '@/theme/colors.css';
import { vars } from '@/theme/theme-vars';

export const navbar = style({ borderBottom: `1px solid ${colors.line}` });

export const nav = style({
  display: 'flex',
  flexWrap: 'wrap',
  alignItems: 'center',
  justifyContent: 'space-between',
  gap: 24,
  paddingBlock: 24,
});

export const brand = style({ display: 'block', color: colors.ink });

export const logo = style({
  display: 'block',
  height: 68,
  width: 'auto',
  '@media': { '(max-width: 640px)': { height: 52 } },
});

export const links = style({
  display: 'flex',
  flexWrap: 'wrap',
  alignItems: 'center',
  gap: 26,
  fontSize: 15,
  fontWeight: 500,
});

export const link = style({
  color: colors.inkSoft,
  textDecoration: 'none',
  ':hover': { color: colors.ink },
});

export const wideOnly = style({
  '@media': { '(max-width: 640px)': { display: 'none' } },
});

export const version = style({
  padding: '2px 8px',
  border: `1px solid ${colors.line}`,
  borderRadius: 999,
  fontFamily: vars.fontFamilyMonospace,
  fontSize: 12,
  color: colors.inkSoft,
});
