import { createGlobalTheme, createThemeContract } from '@vanilla-extract/css';

export const colors = createThemeContract({
  paper: null,
  paperRaised: null,
  ink: null,
  inkSoft: null,
  line: null,
  bamboo: null,
  bambooSoft: null,
  danger: null,
  syntaxVariable: null,
  syntaxOperator: null,
  syntaxWord: null,
  syntaxNumber: null,
  syntaxComment: null,
});

createGlobalTheme(':root[data-mantine-color-scheme="light"]', colors, {
  paper: '#fbfbf9',
  paperRaised: '#f1f2ee',
  ink: '#141513',
  inkSoft: '#555850',
  line: '#dfe1da',
  bamboo: '#3d6a1d',
  bambooSoft: '#e4edda',
  danger: '#b3261e',
  syntaxVariable: '#3d6a1d',
  syntaxOperator: '#b0560a',
  syntaxWord: '#876000',
  syntaxNumber: '#6b3fb3',
  syntaxComment: '#7d8077',
});

createGlobalTheme(':root[data-mantine-color-scheme="dark"]', colors, {
  paper: '#0f100e',
  paperRaised: '#171915',
  ink: '#ecede8',
  inkSoft: '#a4a79e',
  line: '#2a2d27',
  bamboo: '#a6d36f',
  bambooSoft: '#1e2916',
  danger: '#ff8f84',
  syntaxVariable: '#a6d36f',
  syntaxOperator: '#f2a65a',
  syntaxWord: '#e8c58a',
  syntaxNumber: '#c9a7ff',
  syntaxComment: '#7f8379',
});

// The terminal panels stay dark in both color schemes, like the logo's ellipse.
export const panel = {
  background: '#141513',
  raised: '#1f211d',
  text: '#f0f1ec',
  dim: '#9a9d94',
  line: '#2f322c',
  accent: '#a6d36f',
  danger: '#ff8f84',
  syntax: {
    variable: '#a6d36f',
    operator: '#f2a65a',
    word: '#e8c58a',
    number: '#c9a7ff',
    comment: '#7f8379',
    keyword: '#ff9cb4',
    type: '#8fc8ff',
  },
};
