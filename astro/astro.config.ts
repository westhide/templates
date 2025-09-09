import sitemap from "@astrojs/sitemap";
import vue from "@astrojs/vue";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "astro/config";
import compressor from "astro-compressor";
import icon from "astro-icon";
import { FileSystemIconLoader } from "unplugin-icons/loaders";
import Icons from "unplugin-icons/vite";
import { loadEnv } from "vite";

const CWD = process.cwd();

const { PUBLIC_SITE_URL } = loadEnv(process.env.NODE_ENV, CWD, "");

// https://astro.build/config
export default defineConfig({
	site: PUBLIC_SITE_URL,
	output: "static",
	prefetch: {
		defaultStrategy: "viewport",
		prefetchAll: true,
	},
	integrations: [
		vue({
			jsx: true,
			devtools: true,
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
			Icons({
				compiler: "vue3",
				customCollections: {
					collections: FileSystemIconLoader("src/assets/icons"),
				},
			}),
		],
		css: {
			transformer: "lightningcss",
		},
		build: {
			cssMinify: "lightningcss",
		},
	},
});
