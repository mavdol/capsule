/**
 * Capsule SDK - Host API Interface
 *
 * This module provides the interface to call host functions from JavaScript/TypeScript Wasm code.
 * When running in WASM mode, it imports the auto-generated bindings from jco.
 * When running locally, it provides mock implementations for testing.
 */

let hostModule: any = null;
let isWasmChecked = false;
let isWasm = false;

/**
 * Lazily check if we're running in WASM mode.
 */
function checkWasm(): boolean {
  if (!isWasmChecked) {
    try {
      const witBinding = (globalThis as any)["capsule:host/api"];

      if (typeof witBinding !== "undefined" && typeof witBinding.scheduleTask === "function") {
        hostModule = witBinding;
        isWasm = true;
      } else {
        isWasm = false;
      }
    } catch (e) {
      isWasm = false;
    }
    isWasmChecked = true;
  }
  return isWasm;
}

/**
 * Check if running in WASM mode.
 */
export function isWasmMode(): boolean {
  return checkWasm();
}

/**
 * Call the host's schedule_task function to create a new isolated task instance.
 *
 * This is the bridge between JavaScript code and the Rust host runtime.
 */
export function callHost(
  name: string,
  args: any[],
  config: Record<string, any>
): string {
  if (checkWasm() && hostModule !== null) {
    try {
      const result = hostModule.scheduleTask(
        name,
        JSON.stringify(args),
        JSON.stringify(config)
      );
      return result;
    } catch (e) {
      const error = e instanceof Error ? e.message : String(e);
      return JSON.stringify({ error: `Host call failed: ${error}` });
    }
  } else {
    return JSON.stringify({ result: `mock_result_for_${name}` });
  }
}
