import type { FC } from 'react';
import { colored, editor, input } from '@/components/Demo/Editor.css';
import { Syntax } from '@/components/Syntax/Syntax';
import type { SyntaxToken } from '@/lib/highlight/expansion';

type Props = {
  id: string;
  label: string;
  value: string;
  onChange: (value: string) => void;
  highlight: (value: string) => SyntaxToken[];
};

export const Editor: FC<Props> = ({ id, label, value, onChange, highlight }) => (
  <div className={editor}>
    <pre className={colored} aria-hidden>
      <Syntax tokens={highlight(value)} surface="panel" />
      {/* A trailing newline only gets its own line in the copy if something follows it. */}
      {'\n'}
    </pre>
    <textarea
      id={id}
      className={input}
      aria-label={label}
      spellCheck={false}
      value={value}
      onChange={event => onChange(event.target.value)}
    />
  </div>
);
