import type { FC } from 'react';
import { Trans, useTranslation } from 'react-i18next';
import { Container } from '@mantine/core';
import { ConsoleBlock, type ConsoleLine } from '@/components/CodeBlock/ConsoleBlock';
import { RustBlock } from '@/components/CodeBlock/RustBlock';
import {
  body,
  crate,
  flag,
  flagName,
  flagText,
  flags,
  title,
  use,
  uses,
} from '@/components/Usage/Usage.css';
import { CONTAINER, heading, inlineCode, section, wrap } from '@/components/shared/shared.css';

const LIBRARY_EXAMPLE = `use std::collections::HashMap;
use xpanda::Xpanda;

let vars = HashMap::from([
    ("APP".to_string(), "WebShop".to_string()),
]);

let xpanda = Xpanda::builder()
    .with_env_vars()
    .with_named_vars(vars)
    .no_unset(true)
    .build();

assert_eq!(
    xpanda.expand("/srv/\${APP,,}"),
    Ok(String::from("/srv/webshop")),
);`;

const FLAGS = [
  ['-v VAR=value', 'var'],
  ['-f, --var-file', 'varFile'],
  ['-e, --env-vars', 'envVars'],
  ['-u, --no-unset', 'noUnset'],
] as const;

export const Usage: FC = () => {
  const { t } = useTranslation();

  const consoleLines: ConsoleLine[] = [
    { kind: 'comment', text: t('usage.cli.comments.env') },
    { kind: 'command', text: 'VAR=value xpanda < some_file' },
    { kind: 'blank' },
    { kind: 'comment', text: t('usage.cli.comments.file') },
    { kind: 'command', text: 'xpanda -f prod.env -i app.tpl -o app.conf' },
    { kind: 'blank' },
    { kind: 'comment', text: t('usage.cli.comments.positional') },
    { kind: 'command', text: "echo 'Hello ${1:-world}' | xpanda -- Alva" },
    { kind: 'output', text: 'Hello Alva' },
  ];

  return (
    <section id="usage" className={section}>
      <Container size={CONTAINER} className={wrap}>
        <h2 className={heading}>{t('usage.title')}</h2>
        <div className={uses}>
          <div className={use}>
            <h3 className={title}>
              {t('usage.cli.title')} <span className={crate}>xpanda-cli</span>
            </h3>
            <p className={body}>{t('usage.cli.body')}</p>
            <ConsoleBlock lines={consoleLines} />
            <dl className={flags}>
              {FLAGS.map(([name, key]) => (
                <div key={key} className={flag}>
                  <dt>
                    <code className={flagName}>{name}</code>
                  </dt>
                  <dd className={flagText}>{t(`usage.cli.flags.${key}`)}</dd>
                </div>
              ))}
            </dl>
          </div>
          <div className={use}>
            <h3 className={title}>
              {t('usage.library.title')} <span className={crate}>xpanda</span>
            </h3>
            <p className={body}>
              <Trans
                i18nKey="usage.library.body"
                components={{ code: <code className={inlineCode} /> }}
              />
            </p>
            <RustBlock code={LIBRARY_EXAMPLE} />
          </div>
        </div>
      </Container>
    </section>
  );
};
