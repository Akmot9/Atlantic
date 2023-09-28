import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "titre",
  description: "oui",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Documentaion', link: '/Documentaion' }
    ],

    sidebar: [
      {
        text: 'Examples',
        items: [
          { text: 'Documentaion', link: '/Documentaion' },
          { text: 'use-cases', link: '/use-cases' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/vuejs/vitepress' }
    ]
  }
})
