/**
 * Stream polyfill for WASI environment
 * Provides Node.js-compatible streams using the readable-stream package
 */

export {
    Readable,
    Writable,
    Duplex,
    Transform,
    PassThrough,
    Stream,
    pipeline,
    finished,
} from 'readable-stream';

import { Readable, Writable, Duplex, Transform, PassThrough, Stream } from 'readable-stream';

const stream = {
    Readable,
    Writable,
    Duplex,
    Transform,
    PassThrough,
    Stream,
};

export default stream;
