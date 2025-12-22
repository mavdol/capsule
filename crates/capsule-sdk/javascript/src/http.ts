/**
 * Capsule SDK - HTTP Client
 *
 * This module provides HTTP request functions by calling the host's HTTP implementation.
 * In WASM mode, requests go through the Rust host.
 * In local mode, uses native fetch for testing.
 */

import { isWasmMode } from "./hostApi.js";

export interface RequestOptions {
  headers?: Record<string, string>;
  body?: string;
  json?: any;
}

/**
 * HTTP Response wrapper with convenient methods.
 */
export class Response {
  readonly status: number;
  readonly headers: Record<string, string>;
  readonly body: string;

  constructor(status: number, headers: Record<string, string>, body: string) {
    this.status = status;
    this.headers = headers;
    this.body = body;
  }

  /**
   * Parse response body as JSON.
   */
  json<T = any>(): T {
    return JSON.parse(this.body);
  }

  /**
   * Get response body as text.
   */
  text(): string {
    return this.body;
  }

  /**
   * Check if response status is 2xx.
   */
  ok(): boolean {
    return this.status >= 200 && this.status < 300;
  }

  toString(): string {
    return `<Response [${this.status}]>`;
  }
}

/**
 * Internal function to make HTTP requests.
 * In WASM mode: calls the host API
 * In local mode: uses fetch
 */
async function makeRequest(
  method: string,
  url: string,
  options: RequestOptions = {}
): Promise<Response> {
  return isWasmMode()
    ? makeHostRequest(method, url, options)
    : makeLocalRequest(method, url, options);
}

/**
 * Make HTTP request using native fetch (local mode only).
 */
async function makeLocalRequest(
  method: string,
  url: string,
  options: RequestOptions = {}
): Promise<Response> {
  const headers: Record<string, string> = { ...options.headers };
  let body: string | undefined = options.body;

  if (options.json !== undefined) {
    body = JSON.stringify(options.json);
    headers["Content-Type"] = "application/json";
  }

  const fetchOptions: RequestInit = {
    method,
    headers,
  };

  if (body !== undefined) {
    fetchOptions.body = body;
  }

  const fetchResponse = await fetch(url, fetchOptions);
  const responseText = await fetchResponse.text();

  const responseHeaders: Record<string, string> = {};
  fetchResponse.headers.forEach((value, key) => {
    responseHeaders[key] = value;
  });

  return new Response(fetchResponse.status, responseHeaders, responseText);
}

/**
 * Make HTTP request via the Rust host (WASM mode).
 */
function makeHostRequest(
  method: string,
  url: string,
  options: RequestOptions = {}
): Response {
  try {
    const hostModule = (globalThis as any)["capsule:host/api"];

    if (!hostModule || !hostModule.httpRequest) {
      throw new Error("Host HTTP API not available");
    }

    const headers: [string, string][] = [];
    const requestHeaders = options.headers || {};

    for (const [key, value] of Object.entries(requestHeaders)) {
      headers.push([key, value]);
    }

    let body: string | undefined = options.body;

    if (options.json !== undefined) {
      body = JSON.stringify(options.json);
      headers.push(["Content-Type", "application/json"]);
    }

    const result = hostModule.httpRequest(method, url, headers, body);

    const responseHeaders: Record<string, string> = {};
    for (const [key, value] of result.headers) {
      responseHeaders[key] = value;
    }

    return new Response(result.status, responseHeaders, result.body);
  } catch (error) {
    throw new Error(`HTTP request failed: ${error instanceof Error ? error.message : String(error)}`);
  }
}

/**
 * Make an HTTP GET request.
 *
 * @example
 * ```typescript
 * const response = http.get("https://api.github.com/zen");
 * console.log(response.text());
 * ```
 */
export async function get(
  url: string,
  options: Omit<RequestOptions, "body" | "json"> = {}
): Promise<Response> {
  return makeRequest("GET", url, options);
}

/**
 * Make an HTTP POST request.
 *
 * @example
 * ```typescript
 * const response = http.post("https://api.example.com/data", {
 *   json: { key: "value" }
 * });
 * ```
 */
export async function post(
  url: string,
  options: RequestOptions = {}
): Promise<Response> {
  return makeRequest("POST", url, options);
}

/**
 * Make an HTTP PUT request.
 */
export async function put(
  url: string,
  options: RequestOptions = {}
): Promise<Response> {
  return makeRequest("PUT", url, options);
}

/**
 * Make an HTTP DELETE request.
 */
export async function del(
  url: string,
  options: Omit<RequestOptions, "body" | "json"> = {}
): Promise<Response> {
  return makeRequest("DELETE", url, options);
}

/**
 * Make an HTTP PATCH request.
 */
export async function patch(
  url: string,
  options: RequestOptions = {}
): Promise<Response> {
  return makeRequest("PATCH", url, options);
}

/**
 * Make an HTTP HEAD request.
 */
export async function head(
  url: string,
  options: Omit<RequestOptions, "body" | "json"> = {}
): Promise<Response> {
  return makeRequest("HEAD", url, options);
}
