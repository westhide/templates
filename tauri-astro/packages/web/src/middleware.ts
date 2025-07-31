import { defineMiddleware } from "astro:middleware";

export const onRequest = defineMiddleware((_, next) => {
	return next();
});
