import { useState, type FC } from 'react';
import { Trans, useTranslation } from 'react-i18next';
import { Container } from '@mantine/core';
import {
  checkbox,
  description,
  groupCount,
  groupTitle,
  groups,
  option,
  pattern,
  result,
  resultEmpty,
  resultError,
  row,
  tester,
  valueInput,
  valueLabel,
} from '@/components/Patterns/Patterns.css';
import { Syntax } from '@/components/Syntax/Syntax';
import {
  CONTAINER,
  heading,
  inlineCode,
  intro,
  section,
  wrap,
} from '@/components/shared/shared.css';
import { expand } from '@/lib/expand/expand';
import { highlightExpansion } from '@/lib/highlight/expansion';
import { PATTERN_GROUPS } from '@/lib/patterns';

export const Patterns: FC = () => {
  const { t } = useTranslation();
  const [value, setValue] = useState('example');
  const [unset, setUnset] = useState(false);
  const [noUnset, setNoUnset] = useState(false);

  const vars = {
    named: new Map(unset ? [] : [['VAR', value]]),
    positional: [],
    noUnset,
  };

  return (
    <section id="patterns" className={section}>
      <Container size={CONTAINER} className={wrap}>
        <h2 className={heading}>{t('patterns.title')}</h2>
        <p className={intro}>
          <Trans i18nKey="patterns.intro" components={{ code: <code className={inlineCode} /> }} />
        </p>
        <div className={tester}>
          <label className={valueLabel} htmlFor="pattern-value">
            VAR=
            <input
              id="pattern-value"
              type="text"
              className={valueInput}
              spellCheck={false}
              value={value}
              disabled={unset}
              onChange={event => setValue(event.target.value)}
            />
          </label>
          <label className={option}>
            <input
              id="pattern-unset"
              type="checkbox"
              className={checkbox}
              checked={unset}
              onChange={event => setUnset(event.target.checked)}
            />
            {t('patterns.unset')}
          </label>
          <label className={option}>
            <input
              id="pattern-no-unset"
              type="checkbox"
              className={checkbox}
              checked={noUnset}
              onChange={event => setNoUnset(event.target.checked)}
            />
            {t('patterns.noUnset')}
          </label>
        </div>
        <div className={groups}>
          {PATTERN_GROUPS.map(group => (
            <div key={group.id}>
              <h3 className={groupTitle}>
                {t(`patterns.groups.${group.id}`)}
                <span className={groupCount}>
                  {t('patterns.forms', { count: group.rows.length })}
                </span>
              </h3>
              {group.rows.map(entry => {
                const outcome = expand(entry.pattern, vars);
                const text = outcome.ok ? outcome.text : '';
                let className = result;

                if (!outcome.ok) {
                  className = `${result} ${resultError}`;
                } else if (text === '') {
                  className = `${result} ${resultEmpty}`;
                }

                return (
                  <div key={entry.pattern} className={row}>
                    <code className={pattern}>
                      <Syntax tokens={highlightExpansion(entry.pattern)} surface="paper" />
                    </code>
                    <span className={description}>{t(`patterns.rows.${entry.description}`)}</span>
                    <span
                      className={className}
                      title={outcome.ok ? text : outcome.message}
                      data-testid={`result-${entry.pattern}`}
                    >
                      {outcome.ok ? text || t('patterns.empty') : t('patterns.error')}
                    </span>
                  </div>
                );
              })}
            </div>
          ))}
        </div>
      </Container>
    </section>
  );
};
