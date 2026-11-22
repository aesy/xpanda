import type { FC } from 'react';
import { onPanel, onPaper } from '@/components/Syntax/Syntax.css';
import type { SyntaxToken } from '@/lib/highlight/expansion';

type Props = { tokens: SyntaxToken[]; surface: 'paper' | 'panel' };

export const Syntax: FC<Props> = ({ tokens, surface }) => {
  const classes = surface === 'paper' ? onPaper : onPanel;

  return tokens.map((token, index) =>
    token.kind === 'plain' ? (
      token.text
    ) : (
      // Tokens are re-derived from the text on every change, so their position is their identity.
      // eslint-disable-next-line react-x/no-array-index-key
      <span key={index} className={classes[token.kind]}>
        {token.text}
      </span>
    )
  );
};
