/// <reference types="vite/client" />

declare module "*.vue" {
	import type { DefineComponent } from "vue";
	const component: DefineComponent<{}, {}, unknown>;
	export default component;
}

declare namespace NodeJS {
	interface ProcessEnv extends ImportMetaEnv {}
}

export interface ImportMetaEnv {
	readonly PUBLIC_SITE_URL: string;
}

export interface ImportMeta {
	readonly env: ImportMetaEnv;
}
