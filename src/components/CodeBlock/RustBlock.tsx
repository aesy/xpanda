import type { FC } from 'react';
import { block } from '@/components/CodeBlock/CodeBlock.css';
import { HighlightedCode } from '@/components/CodeBlock/HighlightedCode';

type Props = { code: string };

export const RustBlock: FC<Props> = ({ code }) => (
  <pre className={block}>
    <HighlightedCode code={code} language="rust" />
  </pre>
);
