import { ServerConfig, serve } from "ic10lsp_wasm";
import { v4 as uuidv4 } from 'uuid';

export const encoder = new TextEncoder();
export const decoder = new TextDecoder();

export default class Bytes {
  static encode(input) {
    return encoder.encode(input);
  }

  static decode(input) {
    return decoder.decode(input);
  }

  static append(
    constructor,
    ...arrays
  ) {
    let totalLength = 0;
    for (const arr of arrays) {
      totalLength += arr.length;
    }
    const result = new constructor(totalLength);
    let offset = 0;
    for (const arr of arrays) {
      result.set(arr, offset);
      offset += arr.length;
    }
    return result;
  }
}

export class AsyncStreamQueue {

  promises = [];
  resolvers = [];
  observers = [];

  closed = false;
  locked = false;
  tag = "";

  static __add(promises, resolvers) {
    promises.push(new Promise((resolve) => {
      resolvers.push(resolve);
    }))
  }

  static __enqueue(closed, promises, resolvers, item) {
    if (!closed) {
      if (!resolvers.length) AsyncStreamQueue.__add(promises, resolvers);
      const resolve = resolvers.shift();
      resolve(item);
    }
  }

  constructor(tag) {
    this.tag = tag;
    const closed = this.closed;
    // invariant: at least one of the arrays is empty
    const promises = this.promises;
    const resolvers = this.resolvers;
    this.stream = new WritableStream({
      write(item) {
        AsyncStreamQueue.__enqueue(closed, promises, resolvers, item)

      }
    })
  }
  _add() {
    return AsyncStreamQueue.__add(this.promises, this.resolvers);
  }

  enqueue(item) {
    return AsyncStreamQueue.__enqueue(this.closed, this.promises, this.resolvers, item);
  }

  dequeue() {
    if (!this.promises.length) this._add();
    const item = this.promises.shift();
    return item;
  }

  // now some utilities:
  isEmpty() { // there are no values available
    return !this.promises.length; // this.length <= 0
  }

  isBlocked() { // it's waiting for values
    return !!this.resolvers.length; // this.length < 0
  }

  get length() {
    return this.promises.length - this.resolvers.length;
  }

  return() {
    return new Promise(() => { })
  }

  throw(err) {
    return new Promise((_resolve, reject) => {
      reject(err);
    })
  }

  async next() {
    const done = false;
    // console.log(`AsyncStream(${this.tag}) waiting for message`)
    const value = await this.dequeue();
    // console.log(`AsyncStream(${this.tag}) got message`, decoder.decode(value))
    return { done, value };
  }

  [Symbol.asyncIterator]() {
    return this;
  }

  locked() {
    return this.stream.locked;
  }

  abort(reason) {
    return this.stream.abort(reason);
  }

  close() {
    return this.stream.close();
  }

  getWriter() {
    return this.stream.getWriter();
  }

}
let clientMsgStream = new AsyncStreamQueue("client");
let serverMsgStream = new AsyncStreamQueue("server");

async function start() {
  let config = new ServerConfig(clientMsgStream, serverMsgStream);
  await serve(config);
}

function fixup(data) {
  // data.id = uuidv4();
  if (data.hasOwnProperty("params") && data.params.hasOwnProperty("rootUri") && data.params.rootUri === "") {
    data.params.rootUri = null
  }
  if (data.hasOwnProperty("params") && data.params.hasOwnProperty("textDocument")) {
    if (data.params.textDocument.hasOwnProperty("uri")) {
      const match = data.params.textDocument.uri.match(/^file:\/\/\/(.*)/);
      if (null == match) {
        data.params.textDocument.uri = `file:///${data.params.textDocument.uri}`;
      }
    }
    data.params.rootUri = null
  }
  return data
}

function sendClient(data) {
  data = fixup(data);
  const data_j = JSON.stringify(data);
  const msg = `Content-Length: ${data_j.length}\r\n\r\n${data_j}`
  clientMsgStream.enqueue(encoder.encode(msg));
}

async function listen() {
  let contentLength = null;
  let buffer = new Uint8Array();
  console.log("Worker: listening for lsp messages...");
  for await (const bytes of serverMsgStream) {
    buffer = Bytes.append(Uint8Array, buffer, bytes);

    // check if the content length is known
    if (null == contentLength) {
      // if not, try to match the prefixed headers
      const match = Bytes.decode(buffer).match(/^Content-Length:\s*(\d+)\s*/);
      if (null == match) continue;

      // try to parse the content-length from the headers
      const length = parseInt(match[1]);
      if (isNaN(length)) throw new Error("invalid content length");

      // slice the headers since we now have the content length
      buffer = buffer.slice(match[0].length);

      // set the content length
      contentLength = length;
    }

    // if the buffer doesn't contain a full message; await another iteration
    if (buffer.length < contentLength) continue;

    // decode buffer to a string
    const delimited = Bytes.decode(buffer);

    // reset the buffer
    buffer = buffer.slice(contentLength);
    // reset the contentLength
    contentLength = null;

    const message = JSON.parse(delimited);
    console.log("Lsp Message:", message);
    postMessage(message)
  }
  console.log("Worker: lsp message queue done?");
}

listen();

postMessage("ready");

onmessage = function(e) {
  console.log("Client Message:", e.data);
  sendClient(e.data)
}

console.log("Starting LSP...");
start();
