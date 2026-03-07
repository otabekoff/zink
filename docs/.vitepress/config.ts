import { defineConfig } from 'vitepress'
import { createRequire } from 'module'
import { readFileSync } from 'fs'
import { resolve, dirname } from 'path'
import { fileURLToPath } from 'url'

const __dirname = dirname(fileURLToPath(import.meta.url))

// Load the Zink TextMate grammar for Shiki syntax highlighting
const zinkGrammar = JSON.parse(
  readFileSync(resolve(__dirname, '../../lang/editors/zink.tmLanguage.json'), 'utf-8')
)

export default defineConfig({
  title: 'Zink',
  description: 'A fast, readable scripting language built in Rust',
  base: '/zink/',
  head: [
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/zink/favicon.svg' }],
    ['link', { rel: 'icon', type: 'image/png', sizes: '32x32', href: '/zink/favicon.png' }],
    ['link', { rel: 'apple-touch-icon', sizes: '256x256', href: '/zink/icon.png' }],
  ],
  cleanUrls: true,
  markdown: {
    languages: [zinkGrammar],
  },
  themeConfig: {
    logo: { light: '/logo-light.svg', dark: '/logo-dark.svg', alt: 'Zink' },
    siteTitle: 'Zink',
    nav: [
      { text: 'Guide', link: '/guide/getting-started' },
      { text: 'Reference', link: '/reference/syntax' },
      { text: 'Playground', link: '/playground/' },
      {
        text: 'v0.1.0',
        items: [
          { text: 'Changelog', link: '/changelog' },
          { text: 'Contributing', link: '/contributing' },
        ],
      },
    ],
    sidebar: {
      '/guide/': [
        {
          text: 'Introduction',
          items: [
            { text: 'What is Zink?', link: '/guide/what-is-zink' },
            { text: 'Getting Started', link: '/guide/getting-started' },
          ],
        },
        {
          text: 'Basics',
          items: [
            { text: 'Variables', link: '/guide/variables' },
            { text: 'Functions', link: '/guide/functions' },
            { text: 'Control Flow', link: '/guide/control-flow' },
            { text: 'Arrays', link: '/guide/arrays' },
            { text: 'Strings', link: '/guide/strings' },
          ],
        },
        {
          text: 'Advanced',
          items: [
            { text: 'Higher-Order Functions', link: '/guide/higher-order-functions' },
            { text: 'Error Handling', link: '/guide/error-handling' },
          ],
        },
      ],
      '/reference/': [
        {
          text: 'Reference',
          items: [
            { text: 'Syntax', link: '/reference/syntax' },
            { text: 'Built-in Functions', link: '/reference/builtins' },
            { text: 'Operators', link: '/reference/operators' },
            { text: 'Types', link: '/reference/types' },
          ],
        },
      ],
    },
    socialLinks: [
      { icon: 'github', link: 'https://github.com/otabekoff/zink' },
    ],
    editLink: {
      pattern: 'https://github.com/otabekoff/zink/edit/main/docs/:path',
      text: 'Edit this page on GitHub',
    },
    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2026-present Otabek Sadiridinov',
    },
    search: {
      provider: 'local',
    },
  },
})
