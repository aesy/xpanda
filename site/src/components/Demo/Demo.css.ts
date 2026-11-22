import { style } from '@vanilla-extract/css';
import { colors, panel as panelColors } from '@/theme/colors.css';
import { vars } from '@/theme/theme-vars';

export const panel = style({
  minWidth: 0,
  overflow: 'hidden',
  borderRadius: 22,
  background: panelColors.background,
  color: panelColors.text,
  boxShadow: '0 30px 60px -30px rgba(0, 0, 0, 0.45)',
});

export const bar = style({
  display: 'flex',
  flexWrap: 'wrap',
  alignItems: 'center',
  justifyContent: 'space-between',
  gap: 12,
  padding: '12px 18px',
  borderBottom: `1px solid ${panelColors.line}`,
});

const mono = style({ fontFamily: vars.fontFamilyMonospace, fontSize: 13 });

export const caption = style([mono, { color: panelColors.dim }]);

export const toggle = style([
  mono,
  {
    display: 'inline-flex',
    alignItems: 'center',
    gap: 8,
    fontSize: 12,
    color: panelColors.dim,
    cursor: 'pointer',
  },
]);

export const checkbox = style({ accentColor: panelColors.accent });

export const panes = style({
  display: 'grid',
  gridTemplateColumns: 'minmax(0, 3fr) minmax(0, 2fr)',
  '@media': { '(max-width: 640px)': { gridTemplateColumns: 'minmax(0, 1fr)' } },
});

export const pane = style({
  display: 'flex',
  flexDirection: 'column',
  minWidth: 0,
  selectors: {
    '& + &': { borderLeft: `1px solid ${panelColors.line}` },
  },
  '@media': {
    '(max-width: 640px)': {
      selectors: {
        '& + &': { borderLeft: 0, borderTop: `1px solid ${panelColors.line}` },
      },
    },
  },
});

export const paneLabel = style({
  display: 'flex',
  justifyContent: 'space-between',
  gap: 8,
  padding: '12px 18px 0',
  fontSize: 11,
  letterSpacing: '0.08em',
  textTransform: 'uppercase',
  color: panelColors.dim,
});

export const fileName = style([mono, { fontSize: 11, letterSpacing: 0, textTransform: 'none' }]);

export const run = style({
  borderTop: `1px solid ${panelColors.line}`,
  background: panelColors.raised,
});

export const commandLine = style([
  mono,
  {
    padding: '12px 18px 0',
    overflowX: 'auto',
    whiteSpace: 'nowrap',
    color: panelColors.dim,
  },
]);

export const commandPrompt = style({ color: panelColors.accent });

export const exit = style({ fontSize: 12 });

export const exitFailed = style({ color: panelColors.danger });

export const output = style([
  mono,
  {
    minHeight: 150,
    padding: '8px 18px 18px',
    overflowX: 'auto',
    lineHeight: 1.65,
    whiteSpace: 'pre',
  },
]);

export const outputFailed = style({ color: panelColors.danger });

export const hint = style({ marginTop: 12, fontSize: 13, color: colors.inkSoft });
