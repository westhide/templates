declare namespace NodeJS {
	interface ProcessEnv {
		readonly PUBLIC_SITE_URL: string;
	}
}

interface ImportMetaEnv {
	readonly PUBLIC_SITE_URL: string;
}

interface ImportMeta {
	readonly env: ImportMetaEnv;
}
