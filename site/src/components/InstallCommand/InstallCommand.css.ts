import { style } from '@vanilla-extract/css';
import { colors } from '@/theme/colors.css';

export const row = style({
  display: 'flex',
  alignItems: 'center',
  gap: 10,
  minWidth: 0,
  padding: '9px 9px 9px 14px',
  border: `1px solid ${colors.line}`,
  borderRadius: 10,
  background: colors.paperRaised,
  fontSize: 14,
});

export const prompt = style({ color: colors.bamboo, userSelect: 'none' });

export const command = style({
  flex: 1,
  minWidth: 0,
  overflowX: 'auto',
  whiteSpace: 'nowrap',
});

export const copy = style({
  padding: '4px 9px',
  border: `1px solid ${colors.line}`,
  borderRadius: 6,
  background: colors.paper,
  fontSize: 12,
  fontWeight: 500,
  color: colors.inkSoft,
  cursor: 'pointer',
  ':hover': { color: colors.ink },
});
