import type { FC } from 'react';
import { useTranslation } from 'react-i18next';
import { Container } from '@mantine/core';
import { link, links as linkList, logo, row, footer } from '@/components/Footer/Footer.css';
import { Logo } from '@/components/Logo/Logo';
import { CONTAINER, wrap } from '@/components/shared/shared.css';
import { links } from '@/lib/links';

export const Footer: FC = () => {
  const { t } = useTranslation();

  return (
    <footer className={footer}>
      <Container size={CONTAINER} className={`${wrap} ${row}`}>
        <Logo className={logo} label="Xpanda" />
        <div className={linkList}>
          <a className={link} href={links.github}>
            {t('nav.github')}
          </a>
          <a className={link} href={links.crates}>
            {t('footer.crates')}
          </a>
          <a className={link} href={links.docs}>
            {t('footer.docs')}
          </a>
          <a className={link} href={links.comparison}>
            {t('footer.comparison')}
          </a>
        </div>
        <span>{t('footer.license')}</span>
      </Container>
    </footer>
  );
};
