/**
 * Capsule Environment API for WASM environment variables access.
 * Based on wasi:cli/environment@0.2.0
 */

declare const globalThis: {
  'wasi:cli/environment': {
    getEnvironment(): [string, string][];
  };
};

/**
 * Internal helper to safely get the WASI environment bindings.
 */
function getEnvBindings() {
  try {
    const env = globalThis['wasi:cli/environment'];
    if (env && typeof env.getEnvironment === 'function') {
      return env;
    }
  } catch {}
  return null;
}

/**
 * Returns all environment variables as an object.
 * * @returns An object where keys are variable names and values are their values.
 * @example { "NODE_ENV": "production", "PORT": "8080" }
 */
export function getAll(): Record<string, string> {
  const bindings = getEnvBindings();
  if (!bindings) return {};

  try {
    const envList = bindings.getEnvironment();
    return Object.fromEntries(envList);
  } catch (e) {
    console.error("Failed to fetch environment variables:", e);
    return {};
  }
}

/**
 * Get the value of a specific environment variable.
 * * @param key - The name of the environment variable.
 * @returns The value of the variable, or undefined if not found.
 */
export function get(key: string): string | undefined {
  const bindings = getEnvBindings();
  if (!bindings) return undefined;

  try {
    const envList = bindings.getEnvironment();
    const entry = envList.find(([k]) => k === key);
    return entry ? entry[1] : undefined;
  } catch {
    return undefined;
  }
}

/**
 * Check if an environment variable exists.
 * * @param key - The name of the environment variable.
 */
export function has(key: string): boolean {
  return get(key) !== undefined;
}
