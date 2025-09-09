declare namespace NodeJS {
	interface ProcessEnv extends ImportMetaEnv {}
}

export interface ImportMetaEnv {
	readonly PUBLIC_SITE_URL: string;
}

export interface ImportMeta {
	readonly env: ImportMetaEnv;
}
