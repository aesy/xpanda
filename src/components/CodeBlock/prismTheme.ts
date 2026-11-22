import type { PrismTheme } from 'prism-react-renderer';
import { panel } from '@/theme/colors.css';

export const prismTheme: PrismTheme = {
  plain: { color: panel.text, backgroundColor: panel.background },
  styles: [
    { types: ['comment'], style: { color: panel.syntax.comment, fontStyle: 'italic' } },
    { types: ['keyword', 'boolean'], style: { color: panel.syntax.keyword } },
    { types: ['string', 'char'], style: { color: panel.syntax.word } },
    { types: ['number'], style: { color: panel.syntax.number } },
    { types: ['function', 'macro', 'builtin'], style: { color: panel.syntax.variable } },
    { types: ['class-name', 'namespace'], style: { color: panel.syntax.type } },
    { types: ['variable', 'parameter', 'assign-left'], style: { color: panel.syntax.operator } },
    { types: ['punctuation', 'operator'], style: { color: panel.dim } },
  ],
};
