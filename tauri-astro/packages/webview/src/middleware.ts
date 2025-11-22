import { defineMiddleware } from "astro:middleware";
import type { APIContext, MiddlewareNext } from "astro";

function middleware(_ctx: APIContext, next: MiddlewareNext) {
	return next();
}

export const onRequest = defineMiddleware(middleware);
