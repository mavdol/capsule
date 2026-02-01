/**
 * Events polyfill for WASI environment
 * Provides Node.js-compatible EventEmitter using the events package
 */

import EventEmitter from 'events';

(globalThis as any).EventEmitter = EventEmitter;

export { EventEmitter };
export default EventEmitter;
