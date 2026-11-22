import type { FC } from 'react';
import { Highlight } from 'prism-react-renderer';
import { prismTheme } from '@/components/CodeBlock/prismTheme';
import { Prism } from '@/lib/prism';

type Props = { code: string; language: 'rust' | 'bash' };

// Renders the tokens only, so callers can place them inside their own <pre>.
export const HighlightedCode: FC<Props> = ({ code, language }) => (
  <Highlight prism={Prism} theme={prismTheme} code={code} language={language}>
    {({ tokens, getLineProps, getTokenProps }) =>
      tokens.map((line, lineIndex) => (
        // Lines and tokens never reorder, so their position is their identity.
        // eslint-disable-next-line react-x/no-array-index-key
        <span key={lineIndex} {...getLineProps({ line })} style={undefined}>
          {line.map((token, tokenIndex) => (
            // eslint-disable-next-line react-x/no-array-index-key
            <span key={tokenIndex} {...getTokenProps({ token })} />
          ))}
          {lineIndex < tokens.length - 1 ? '\n' : null}
        </span>
      ))
    }
  </Highlight>
);
