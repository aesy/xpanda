import { useState, type FC } from 'react';
import { Trans, useTranslation } from 'react-i18next';
import {
  bar,
  caption,
  checkbox,
  commandLine,
  commandPrompt,
  exit,
  exitFailed,
  fileName,
  hint,
  output,
  outputFailed,
  pane,
  paneLabel,
  panel,
  panes,
  run,
  toggle,
} from '@/components/Demo/Demo.css';
import { Editor } from '@/components/Demo/Editor';
import { inlineCode } from '@/components/shared/shared.css';
import { parseEnvFile } from '@/lib/envFile';
import { expand } from '@/lib/expand/expand';
import { highlightEnvFile, highlightTemplate } from '@/lib/highlight/expansion';

const TEMPLATE = `server {
  listen \${PORT:-8080};
  server_name \${HOST:?HOST must be set};
  root /srv/\${APP,,}/public;

  # v\${VERSION#v} deployed by \${USER^}
  add_header X-Env "\${ENV:-dev}";
  add_header X-Build "\${COMMIT:0:7}";
}`;

const VARS = `HOST=shop.example.org
APP=WebShop
VERSION=v2.4.1
USER=alva
COMMIT=9f3c2e1d84b7a05`;

export const Demo: FC = () => {
  const { t } = useTranslation();
  const [template, setTemplate] = useState(TEMPLATE);
  const [vars, setVars] = useState(VARS);
  const [noUnset, setNoUnset] = useState(false);

  const result = expand(template, { named: parseEnvFile(vars), positional: [], noUnset });

  return (
    <div>
      <section className={panel} aria-label={t('demo.label')}>
        <div className={bar}>
          <span className={caption}>{t('demo.caption')}</span>
          <label className={toggle}>
            <input
              id="demo-no-unset"
              type="checkbox"
              className={checkbox}
              checked={noUnset}
              onChange={event => setNoUnset(event.target.checked)}
            />
            -u ({t('demo.noUnset')})
          </label>
        </div>
        <div className={panes}>
          <div className={pane}>
            <div className={paneLabel}>
              <span>{t('demo.template')}</span>
              <span className={fileName}>nginx.conf.tpl</span>
            </div>
            <Editor
              id="demo-template"
              label={t('demo.template')}
              value={template}
              onChange={setTemplate}
              highlight={highlightTemplate}
            />
          </div>
          <div className={pane}>
            <div className={paneLabel}>
              <span>{t('demo.variables')}</span>
              <span className={fileName}>vars.env</span>
            </div>
            <Editor
              id="demo-vars"
              label={t('demo.variables')}
              value={vars}
              onChange={setVars}
              highlight={highlightEnvFile}
            />
          </div>
        </div>
        <div className={run}>
          <div className={commandLine}>
            <span className={commandPrompt}>$</span> xpanda {noUnset ? '-u ' : ''}-f vars.env &lt;
            nginx.conf.tpl{' '}
            <span className={result.ok ? exit : `${exit} ${exitFailed}`}>
              exit {result.ok ? 0 : 1}
            </span>
          </div>
          <pre
            className={result.ok ? output : `${output} ${outputFailed}`}
            aria-label={t('demo.output')}
            aria-live="polite"
          >
            {result.ok ? result.text : `${result.line}:${result.column} ${result.message}`}
          </pre>
        </div>
      </section>
      <p className={hint}>
        <Trans i18nKey="demo.hint" components={{ code: <code className={inlineCode} /> }} />
      </p>
    </div>
  );
};
