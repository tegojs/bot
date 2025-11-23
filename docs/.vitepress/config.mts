import { defineConfig } from "vitepress";

export default defineConfig({
  title: "@tego/bot",
  description:
    "High-performance desktop automation library for Node.js, powered by Rust",
  base: "/bot/",

  // Exclude api directory from VitePress processing
  srcExclude: ["api/**"],

  themeConfig: {
    logo: "/logo.svg",

    nav: [
      { text: "Home", link: "/" },
      { text: "API", link: "/api/" },
      { text: "Development", link: "/developments/" },
      { text: "GitHub", link: "https://github.com/tegojs/bot" },
    ],

    sidebar: {
      "/api/": [
        {
          text: "API Documentation",
          items: [
            { text: "Overview", link: "/api/" },
            { text: "Full API Reference", link: "/api/index.html" },
          ],
        },
      ],
      "/developments/": [
        {
          text: "Development Notes",
          items: [
            { text: "Overview", link: "/developments/" },
            {
              text: "AutoHotkey API",
              link: "/developments/autohotkey-api-reference",
            },
            {
              text: "Hammerspoon API",
              link: "/developments/hammerspoon-api-reference",
            },
            {
              text: "Python Libraries",
              link: "/developments/python-automation-libraries",
            },
            {
              text: "Rust 2024 Migration",
              link: "/developments/rust-2024-edition-migration",
            },
            {
              text: "Rust 2024 Features",
              link: "/developments/rust-2024-features-analysis",
            },
            {
              text: "CI Integration Tests",
              link: "/developments/ci-integration-tests",
            },
          ],
        },
      ],
    },

    socialLinks: [{ icon: "github", link: "https://github.com/tegojs/bot" }],

    footer: {
      message: "Released under the MIT License.",
      copyright: "Copyright © 2024-present sealday",
    },

    search: {
      provider: "local",
    },
  },

  vite: {
    server: {
      fs: {
        // Allow serving files from api directory
        allow: [".."],
      },
    },
  },
});
