import { defineConfig } from "astro/config";

import sitemap from "@astrojs/sitemap";
import vue from "@astrojs/vue";
import tailwindcss from "@tailwindcss/vite";
import compressor from "astro-compressor";
import icon from "astro-icon";
import imagemin from "unplugin-imagemin/vite";

// https://astro.build/config
export default defineConfig({
    site: process.env.PUBLIC_SITE_URL,
    prefetch: {
        defaultStrategy: "viewport",
        prefetchAll: true,
    },
    integrations: [
        vue({ jsx: true }),
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
        plugins: [tailwindcss(), imagemin()],
        css: {
            transformer: "lightningcss",
        },
        build: {
            cssMinify: "lightningcss",
        },
    },
});
