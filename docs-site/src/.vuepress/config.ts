import { defineUserConfig } from "vuepress";
import { hopeTheme } from "vuepress-theme-hope";

export default defineUserConfig({
  base: "/audio-ninja/",
  lang: "en-US",
  title: "Audio Ninja",
  description: "Wireless Immersive Audio Platform with IAMF Support",
  head: [
    ["meta", { name: "theme-color", content: "#3eaf7c" }],
    ["meta", { name: "apple-mobile-web-app-capable", content: "yes" }],
    ["meta", { name: "apple-mobile-web-app-status-bar-style", content: "black" }],
  ],
  
  theme: hopeTheme({
    logo: "/logo.svg",
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
        ],
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
        ],
      },
      {
        text: "Deployment",
        icon: "server",
        prefix: "/deployment/",
        children: [
          "daemon.md",
          "cli.md",
          "api.md",
          "firmware.md",
          "release.md",
        ],
      },
      {
        text: "Reference",
        icon: "book-bookmark",
        link: "/reference/",
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
          ],
        },
      ],
      "/deployment/": [
        {
          text: "Production Deployment",
          icon: "rocket",
          children: [
            "daemon.md",
            "cli.md",
            "api.md",
            "firmware.md",
            "release.md",
          ],
        },
      ],
      "/reference/": [
        {
          text: "Technical Reference",
          icon: "book-bookmark",
          children: [
            "README.md",
            "api_usage.md",
            "calibration.md",
            "codec_integration.md",
            "daemon_workflow.md",
            "firmware_update.md",
            "hoa.md",
            "hrtf.md",
            "loudness_drc.md",
            "release.md",
            "vbap.md",
          ],
        },
      ],
    },
    
    footer: {
      display: true,
      content: "Apache 2.0 Licensed | Copyright © 2024-2025 Audio Ninja Contributors",
    },
    
    plugins: {
      searchPro: true,
      shiki: {
        lineNumbers: false,
        notationHighlight: true,
      },
    },
  }),
});
