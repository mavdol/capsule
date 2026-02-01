/**
 * stream/web polyfill for WASI environment
 * Exports the native Web Streams API which is built into StarlingMonkey
 */

export const ReadableStream = globalThis.ReadableStream;
export const WritableStream = globalThis.WritableStream;
export const TransformStream = globalThis.TransformStream;
export const ReadableStreamDefaultReader = globalThis.ReadableStreamDefaultReader;
export const ReadableStreamBYOBReader = (globalThis as any).ReadableStreamBYOBReader;
export const WritableStreamDefaultWriter = globalThis.WritableStreamDefaultWriter;
export const ByteLengthQueuingStrategy = globalThis.ByteLengthQueuingStrategy;
export const CountQueuingStrategy = globalThis.CountQueuingStrategy;

export default {
    ReadableStream,
    WritableStream,
    TransformStream,
    ReadableStreamDefaultReader,
    ReadableStreamBYOBReader,
    WritableStreamDefaultWriter,
    ByteLengthQueuingStrategy,
    CountQueuingStrategy,
};
