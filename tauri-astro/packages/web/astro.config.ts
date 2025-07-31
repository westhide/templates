import { defineConfig } from "astro/config";

import sitemap from "@astrojs/sitemap";
import vue from "@astrojs/vue";
import tailwindcss from "@tailwindcss/vite";
import compressor from "astro-compressor";
import icon from "astro-icon";
import imagemin from "unplugin-imagemin/vite";
import wasm from "vite-plugin-wasm";

// https://astro.build/config
export default defineConfig({
    site: process.env.PUBLIC_SITE_URL,
    output: 'static',
    prefetch: {
        defaultStrategy: "viewport",
        prefetchAll: true,
    },
    integrations: [
        vue({
            jsx: true,
            devtools: true
        }),
        icon({
            iconDir: "src/assets/icons",
        }),
        sitemap(),
        compressor(),
    ],
    experimental: {
        clientPrerender: true,
        contentIntellisense: true,
        preserveScriptOrder: true,
    },
    vite: {
        plugins: [
            tailwindcss(),
            wasm(),
            imagemin(),
        ],
        css: {
            transformer: "lightningcss",
        },
        build: {
            cssMinify: "lightningcss",
        },
    },
});
