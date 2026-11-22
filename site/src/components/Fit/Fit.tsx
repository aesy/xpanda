import type { FC, ReactNode } from 'react';
import { Trans, useTranslation } from 'react-i18next';
import { Container } from '@mantine/core';
import {
  bad,
  body,
  danger,
  example,
  good,
  name,
  ours,
  reach,
  scale,
  spectrum,
  spot,
  track,
  verdict,
} from '@/components/Fit/Fit.css';
import {
  CONTAINER,
  heading,
  inlineCode,
  intro,
  section,
  wrap,
} from '@/components/shared/shared.css';
import { PATTERN_COUNT } from '@/lib/patterns';

type SpotProps = {
  tool: keyof typeof reach;
  name: string;
  scaleLabel: string;
  text: ReactNode;
  input: string;
  result: ReactNode;
  works: boolean;
  verdictText: string;
};

const Spot: FC<SpotProps> = props => (
  <div className={props.tool === 'xpanda' ? `${spot} ${ours}` : spot}>
    <h3 className={name}>{props.name}</h3>
    <div className={scale}>
      <i className={`${track} ${reach[props.tool]}`} />
      {props.scaleLabel}
    </div>
    <p className={body}>{props.text}</p>
    <div className={example}>
      {props.input} <span className={props.works ? good : bad}>&#8594; {props.result}</span>
    </div>
    <div className={verdict}>{props.verdictText}</div>
  </div>
);

export const Fit: FC = () => {
  const { t } = useTranslation();
  const code = <code className={inlineCode} />;

  return (
    <section id="fit" className={section}>
      <Container size={CONTAINER} className={wrap}>
        <h2 className={heading}>{t('fit.title')}</h2>
        <p className={intro}>{t('fit.intro')}</p>
        <div className={spectrum}>
          <Spot
            tool="envsubst"
            name="envsubst"
            scaleLabel={t('fit.forms', { count: 2 })}
            text={<Trans i18nKey="fit.envsubst.body" components={{ code }} />}
            input="listen ${PORT:-8080};"
            result="listen ${PORT:-8080};"
            works={false}
            verdictText={t('fit.envsubst.verdict')}
          />
          <Spot
            tool="xpanda"
            name="xpanda"
            scaleLabel={t('fit.forms', { count: PATTERN_COUNT })}
            text={t('fit.xpanda.body')}
            input="listen ${PORT:-8080};"
            result="listen 8080;"
            works
            verdictText={t('fit.xpanda.verdict')}
          />
          <Spot
            tool="eval"
            name="bash eval"
            scaleLabel={t('fit.everything')}
            text={
              <Trans
                i18nKey="fit.eval.body"
                components={{ code, danger: <strong className={danger} /> }}
              />
            }
            input={'eval "echo \\"$(cat tpl)\\""'}
            result={t('fit.eval.result')}
            works={false}
            verdictText={t('fit.eval.verdict')}
          />
        </div>
      </Container>
    </section>
  );
};
