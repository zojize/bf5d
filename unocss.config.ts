import { defineConfig } from 'unocss';
import presetUno from '@unocss/preset-uno';
import { icons as mdi } from '@iconify-json/mdi';
import presetIcons from '@unocss/preset-icons';

export default defineConfig({
  presets: [
    presetUno(),
    presetIcons({
      collections: { mdi },
    }),
  ],
  rules: [
    [
      /^(w|h)-content$/,
      ([, match]) => ({
        [{ w: 'width', h: 'height' }[match as 'w' | 'h']]: 'fit-content',
      }),
    ],
    [/^content-\[(.*)\]$/, ([, content]) => ({ content })],
  ],
  variants: [
    (matcher) =>
      matcher.startsWith('scrollbar-thumb-')
        ? {
            matcher: matcher.substring(16),
            selector: (s) => `${s}::-webkit-scrollbar-thumb`,
          }
        : { matcher },
    (matcher) =>
      matcher.startsWith('scrollbar-track-')
        ? {
            matcher: matcher.substring(16),
            selector: (s) => `${s}::-webkit-scrollbar-track`,
          }
        : { matcher },
    (matcher) =>
      matcher.startsWith('scrollbar-')
        ? {
            matcher: matcher.substring(10),
            selector: (s) => `${s}::-webkit-scrollbar`,
          }
        : { matcher },
    (matcher) =>
      matcher.startsWith('marker:')
        ? {
            matcher: matcher.substring(7),
            selector: (s) => `${s}::marker`,
          }
        : { matcher },
  ],
  theme: {
    fontFamily: {
      cascadia: [
        // credits to typescript playground default font
        'Cascadia',
        'Menlo',
        'Monaco',
        'Consolas',
        'monospace',
      ].join(','),
    },
  },
});
