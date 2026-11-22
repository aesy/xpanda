import { styleVariants } from '@vanilla-extract/css';
import { colors, panel } from '@/theme/colors.css';

export const onPaper = styleVariants({
  plain: {},
  variable: { color: colors.syntaxVariable },
  operator: { color: colors.syntaxOperator },
  word: { color: colors.syntaxWord },
  number: { color: colors.syntaxNumber },
  comment: { color: colors.syntaxComment, fontStyle: 'italic' },
});

export const onPanel = styleVariants({
  plain: {},
  variable: { color: panel.syntax.variable },
  operator: { color: panel.syntax.operator },
  word: { color: panel.syntax.word },
  number: { color: panel.syntax.number },
  comment: { color: panel.syntax.comment, fontStyle: 'italic' },
});
