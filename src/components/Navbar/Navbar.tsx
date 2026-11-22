import type { FC } from 'react';
import { useTranslation } from 'react-i18next';
import { Container } from '@mantine/core';
import { Logo } from '@/components/Logo/Logo';
import {
  brand,
  link,
  links as linkList,
  logo,
  nav,
  navbar,
  version,
  wideOnly,
} from '@/components/Navbar/Navbar.css';
import { CONTAINER, wrap } from '@/components/shared/shared.css';
import { links } from '@/lib/links';

export const Navbar: FC = () => {
  const { t } = useTranslation();

  return (
    <header className={navbar}>
      <Container size={CONTAINER} className={wrap}>
        <nav aria-label={t('nav.label')} className={nav}>
          <a className={brand} href="#top" aria-label={t('nav.home')}>
            <Logo className={logo} />
          </a>
          <div className={linkList}>
            <a className={link} href="#patterns">
              {t('nav.patterns')}
            </a>
            <a className={`${link} ${wideOnly}`} href="#usage">
              {t('nav.usage')}
            </a>
            <a className={link} href={links.docs}>
              {t('nav.docs')}
            </a>
            <a className={link} href={links.github}>
              {t('nav.github')}
            </a>
            <span className={version}>v{__XPANDA_VERSION__}</span>
          </div>
        </nav>
      </Container>
    </header>
  );
};
