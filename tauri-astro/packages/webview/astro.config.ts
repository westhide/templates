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

// @ts-expect-error
const { PUBLIC_SITE_URL } = loadEnv(process.env.NODE_ENV, CWD, "");

// @ts-expect-error
const HOST = process.env.TAURI_DEV_HOST;

// https://astro.build/config
export default defineConfig({
	site: PUBLIC_SITE_URL,
	output: "static",
	prefetch: {
		defaultStrategy: "viewport",
		prefetchAll: true,
	},
	server: {
		host: HOST || false,
		port: 1420,
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
			// @ts-expect-error
			tailwindcss(),
			// @ts-expect-error
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
