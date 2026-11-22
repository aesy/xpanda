import type { FC } from 'react';
import { LOGO_PATH, LOGO_TRANSFORM, LOGO_VIEW_BOX } from '@/components/Logo/logoPath';

type Props = { className?: string; label?: string };

export const Logo: FC<Props> = ({ className, label }) => (
  <svg
    className={className}
    viewBox={LOGO_VIEW_BOX}
    role={label ? 'img' : undefined}
    aria-label={label}
    aria-hidden={label ? undefined : true}
  >
    <g transform={LOGO_TRANSFORM} fill="currentColor">
      <path d={LOGO_PATH} />
    </g>
  </svg>
);
