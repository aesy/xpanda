import type { FC } from 'react';
import { block, comment, output, prompt } from '@/components/CodeBlock/CodeBlock.css';
import { HighlightedCode } from '@/components/CodeBlock/HighlightedCode';

export type ConsoleLine =
  | { kind: 'comment'; text: string }
  | { kind: 'command'; text: string }
  | { kind: 'output'; text: string }
  | { kind: 'blank' };

type Props = { lines: ConsoleLine[] };

function renderLine(line: ConsoleLine) {
  switch (line.kind) {
    case 'comment':
      return <span className={comment}># {line.text}</span>;
    case 'command':
      return (
        <>
          <span className={prompt}>$</span> <HighlightedCode code={line.text} language="bash" />
        </>
      );
    case 'output':
      return <span className={output}>{line.text}</span>;
    case 'blank':
      return null;
  }
}

export const ConsoleBlock: FC<Props> = ({ lines }) => (
  <pre className={block}>
    {lines.map((line, index) => (
      // The lines are fixed content, so their position is their identity.
      // eslint-disable-next-line react-x/no-array-index-key
      <span key={index}>
        {renderLine(line)}
        {index < lines.length - 1 ? '\n' : null}
      </span>
    ))}
  </pre>
);
