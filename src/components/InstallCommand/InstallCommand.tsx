import { useRef, useState, type FC } from 'react';
import { useTranslation } from 'react-i18next';
import { useTimeout } from '@mantine/hooks';
import {
  command as commandText,
  copy,
  prompt,
  row,
} from '@/components/InstallCommand/InstallCommand.css';

type Props = { command: string };

export const InstallCommand: FC<Props> = ({ command }) => {
  const { t } = useTranslation();
  const [copied, setCopied] = useState(false);
  const code = useRef<HTMLElement>(null);
  const reset = useTimeout(() => setCopied(false), 1400);

  const selectCommand = () => {
    const selection = window.getSelection();

    if (code.current && selection) {
      selection.selectAllChildren(code.current);
    }
  };

  const onCopy = () => {
    navigator.clipboard.writeText(command).then(() => {
      setCopied(true);
      reset.start();
    }, selectCommand);
  };

  return (
    <div className={row}>
      <span className={prompt} aria-hidden>
        $
      </span>
      <code ref={code} className={commandText}>
        {command}
      </code>
      <button type="button" className={copy} onClick={onCopy}>
        {copied ? t('hero.copied') : t('hero.copy')}
      </button>
    </div>
  );
};
