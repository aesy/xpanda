import type { FC } from 'react';
import { Trans, useTranslation } from 'react-i18next';
import { Container } from '@mantine/core';
import { Demo } from '@/components/Demo/Demo';
import {
  ctas,
  facts,
  ghostButton,
  grid,
  installs,
  lede,
  solidButton,
  title,
  titleLine,
  titleToken,
} from '@/components/Hero/Hero.css';
import { InstallCommand } from '@/components/InstallCommand/InstallCommand';
import { Syntax } from '@/components/Syntax/Syntax';
import { CONTAINER, inlineCode, wrap } from '@/components/shared/shared.css';
import { highlightExpansion } from '@/lib/highlight/expansion';
import { links } from '@/lib/links';
import { PATTERN_COUNT } from '@/lib/patterns';

export const Hero: FC = () => {
  const { t } = useTranslation();

  return (
    <section>
      <Container size={CONTAINER} className={`${wrap} ${grid}`}>
        <div>
          <h1 className={title}>
            <span className={titleLine}>{t('hero.titleStart')}</span>
            <span className={titleLine}>
              {t('hero.titleEnd')}{' '}
              <span className={titleToken}>
                <Syntax tokens={highlightExpansion('${VAR:-safe}')} surface="paper" />
              </span>
            </span>
          </h1>
          <p className={lede}>
            <Trans i18nKey="hero.lede" components={{ code: <code className={inlineCode} /> }} />
          </p>
          <div className={ctas}>
            <a className={solidButton} href={links.releases}>
              {t('hero.download')}
            </a>
            <a className={ghostButton} href={links.docs}>
              {t('hero.docs')}
            </a>
          </div>
          <div className={installs}>
            <InstallCommand command="cargo add xpanda" />
            <InstallCommand command="cargo install xpanda-cli" />
          </div>
          <div className={facts}>
            <span>{t('hero.license')}</span>
            <span>{t('hero.msrv', { version: __XPANDA_MSRV__ })}</span>
            <span>{t('hero.forms', { count: PATTERN_COUNT })}</span>
          </div>
        </div>
        <Demo />
      </Container>
    </section>
  );
};
