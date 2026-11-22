import { style } from '@vanilla-extract/css';
import { panel } from '@/theme/colors.css';
import { vars } from '@/theme/theme-vars';

export const block = style({
  margin: 0,
  padding: '18px 20px',
  overflowX: 'auto',
  borderRadius: 14,
  background: panel.background,
  color: panel.text,
  fontFamily: vars.fontFamilyMonospace,
  fontSize: 13,
  lineHeight: 1.7,
});

export const prompt = style({ color: panel.accent, userSelect: 'none' });

export const comment = style({ color: panel.syntax.comment, fontStyle: 'italic' });

export const output = style({ color: panel.dim });
