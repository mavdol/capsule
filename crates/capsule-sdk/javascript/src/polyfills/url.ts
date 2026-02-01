/**
 * URL polyfill for WASI environment
 * Aliases Node.js 'url' module to native Web URL APIs
 */

export const URL = globalThis.URL;

export const URLSearchParams = globalThis.URLSearchParams;

/**
 * Legacy Node.js url.parse()
 */
export function parse(urlString: string, parseQueryString?: boolean): any {
    try {
        const url = new URL(urlString);
        return {
            href: url.href,
            protocol: url.protocol,
            host: url.host,
            hostname: url.hostname,
            port: url.port,
            pathname: url.pathname,
            search: url.search,
            hash: url.hash,
            query: parseQueryString ? Object.fromEntries(url.searchParams) : url.search.slice(1),
        };
    } catch (e) {
        return null;
    }
}

/**
 * Legacy Node.js url.format()
 */
export function format(urlObject: any): string {
    if (typeof urlObject === 'string') {
        return urlObject;
    }

    try {
        const protocol = urlObject.protocol || 'http:';
        const hostname = urlObject.hostname || urlObject.host || 'localhost';
        const port = urlObject.port ? `:${urlObject.port}` : '';
        const pathname = urlObject.pathname || '/';
        const search = urlObject.search || '';
        const hash = urlObject.hash || '';

        return `${protocol}//${hostname}${port}${pathname}${search}${hash}`;
    } catch (e) {
        return '';
    }
}

/**
 * Legacy Node.js url.resolve()
 */
export function resolve(from: string, to: string): string {
    try {
        return new URL(to, from).href;
    } catch (e) {
        return to;
    }
}

const url = {
    URL,
    URLSearchParams,
    parse,
    format,
    resolve,
};

export default url;
