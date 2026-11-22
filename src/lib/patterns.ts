export type PatternRow = {
  pattern: string;
  description: string;
  // Another example of a form already listed, so not counted as a form of its own.
  variant?: true;
};

export type PatternGroup = { id: string; rows: PatternRow[] };

export const PATTERN_GROUPS: PatternGroup[] = [
  {
    id: 'plain',
    rows: [
      { pattern: '$VAR', description: 'simple' },
      { pattern: '${VAR}', description: 'braced' },
    ],
  },
  {
    id: 'defaults',
    rows: [
      { pattern: '${VAR-default}', description: 'default' },
      { pattern: '${VAR:-default}', description: 'defaultEmpty' },
      { pattern: '${VAR+alt}', description: 'alt' },
      { pattern: '${VAR:+alt}', description: 'altEmpty' },
      { pattern: '${VAR?missing}', description: 'error' },
      { pattern: '${VAR:?missing}', description: 'errorEmpty' },
    ],
  },
  {
    id: 'length',
    rows: [
      { pattern: '${#VAR}', description: 'length' },
      { pattern: '${!VAR}', description: 'ref' },
      { pattern: '${#}', description: 'count' },
    ],
  },
  {
    id: 'substrings',
    rows: [
      { pattern: '${VAR:2}', description: 'offset' },
      { pattern: '${VAR:2:4}', description: 'offsetLength' },
      { pattern: '${VAR: -3}', description: 'negative', variant: true },
    ],
  },
  {
    id: 'case',
    rows: [
      { pattern: '${VAR^}', description: 'upperFirst' },
      { pattern: '${VAR^^}', description: 'upperAll' },
      { pattern: '${VAR,}', description: 'lowerFirst' },
      { pattern: '${VAR,,}', description: 'lowerAll' },
      { pattern: '${VAR~}', description: 'reverseFirst' },
      { pattern: '${VAR~~}', description: 'reverseAll' },
    ],
  },
  {
    id: 'trim',
    rows: [
      { pattern: '${VAR#*e}', description: 'prefixShort' },
      { pattern: '${VAR##*e}', description: 'prefixLong' },
      { pattern: '${VAR%e*}', description: 'suffixShort' },
      { pattern: '${VAR%%e*}', description: 'suffixLong' },
      { pattern: '${VAR/e/E}', description: 'replaceFirst' },
      { pattern: '${VAR//e/E}', description: 'replaceAll' },
    ],
  },
];

export const PATTERN_COUNT = PATTERN_GROUPS.flatMap(group => group.rows).filter(
  row => !row.variant
).length;
