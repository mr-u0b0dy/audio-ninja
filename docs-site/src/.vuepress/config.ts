import { defineUserConfig } from "vuepress";
import { viteBundler } from "@vuepress/bundler-vite";
import { hopeTheme } from "vuepress-theme-hope";

export default defineUserConfig({
  bundler: viteBundler(),
  base: "/audio-ninja/",
  lang: "en-US",
  title: "Audio Ninja",
  description: "Wireless Immersive Audio Platform with IAMF Support",
  head: [
    ["meta", { name: "theme-color", content: "#E65100" }],
    ["meta", { name: "apple-mobile-web-app-capable", content: "yes" }],
    ["meta", { name: "apple-mobile-web-app-status-bar-style", content: "black" }],
  ],

  theme: hopeTheme({
    themeColor: {
      "#E65100": "Magma Orange",
      "#FF8C00": "Neon Amber",
      "#FFD580": "Blade Glow",
    },
    logo: "/logo.png",
    repo: "mr-u0b0dy/audio-ninja",
    repoLabel: "GitHub",
    docsDir: "src",
    docsBranch: "docs/vuepress-theme-hope",
    editLink: true,
    editLinkPattern: ":repo/edit/:branch/:path",

    navbar: [
      "/",
      {
        text: "Guide",
        icon: "book",
        prefix: "/guide/",
        children: [
          "README.md",
          "quick-start.md",
          "installation.md",
          "configuration.md",
          "cli-tui.md",
          "tui.md",
          "gui-quick-reference.md",
        ],
      },
      {
        text: "GUI Design",
        icon: "palette",
        prefix: "/design/",
        children: [
          "README.md",
          "design-system.md",
          "phase2-tasks.md",
        ],
      },
      {
        text: "Roadmap & Todos",
        icon: "list",
        link: "/todo/",
      },
      {
        text: "Spatial Audio",
        icon: "headphones",
        prefix: "/spatial/",
        children: [
          "overview.md",
          "vbap.md",
          "hoa.md",
          "hrtf.md",
          "comparison.md",
        ],
      },
      {
        text: "Audio Processing",
        icon: "sliders",
        prefix: "/processing/",
        children: [
          "loudness.md",
          "drc.md",
          "calibration.md",
          "codecs.md",
          "codec_integration.md",
          "firmware_update.md",
        ],
      },
      {
        text: "API Reference",
        icon: "code",
        link: "/api/",
      },
    ],

    sidebar: {
      "/guide/": [
        {
          text: "Getting Started",
          icon: "lightbulb",
          children: [
            "README.md",
            "quick-start.md",
            "installation.md",
            "configuration.md",
            "cli-tui.md",
            "tui.md",
            "gui-quick-reference.md",
          ],
        },
      ],
      "/design/": [
        {
          text: "GUI Design & Implementation",
          icon: "palette",
          children: [
            "README.md",
            "design-system.md",
            "phase2-tasks.md",
          ],
        },
      ],
      "/spatial/": [
        {
          text: "3D Spatial Rendering",
          icon: "cube",
          children: [
            "overview.md",
            "vbap.md",
            "hoa.md",
            "hrtf.md",
            "comparison.md",
          ],
        },
      ],
      "/processing/": [
        {
          text: "DSP & Audio Processing",
          icon: "waveform-lines",
          children: [
            "loudness.md",
            "drc.md",
            "calibration.md",
            "codecs.md",
            "codec_integration.md",
            "firmware_update.md",
          ],
        },
      ],
      "/api/": [
        {
          text: "API & Integration",
          icon: "code",
          children: [
            "README.md",
            "reference.md",
            "api_usage.md",
            "daemon_workflow.md",
            "release.md",
          ],
        },
      ],
      "/todo/": [
        {
          text: "Project Roadmap",
          icon: "list",
          children: [
            "README.md",
          ],
        },
      ],
      "/": [],
    },

    displayFooter: true,
    footer: "Apache 2.0 Licensed | Copyright © 2024-2025 Audio Ninja Contributors",

    plugins: {
      shiki: {
        lineNumbers: false,
        notationHighlight: true,
      },
    },
  }),
});
