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
    ["meta", { name: "theme-color", content: "#0F766E" }],
    ["meta", { name: "apple-mobile-web-app-capable", content: "yes" }],
    ["meta", { name: "apple-mobile-web-app-status-bar-style", content: "black" }],
  ],
  
  theme: hopeTheme({
    themeColor: {
      "#0F766E": "Teal",
      "#14B8A6": "Mint",
      "#F59E0B": "Amber",
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
            "api_usage.md",
            "daemon_workflow.md",
            "release.md",
          ],
        },
      ],
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
