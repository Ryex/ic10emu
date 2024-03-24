

async function setDocFragment(content) {
  const obj = JSON.stringify({ sessions: [{ content }] })
  const bytes = new TextEncoder().encode(obj);
  try {
    const c_bytes = await compress(bytes);
    const fragment = base64url_encode(c_bytes);
    window.history.replaceState(null, "", `#${fragment}`);
  } catch (e) {
    console.log("Error compressing content fragment:", e);
    return;
  }

}

async function decompressFragment(c_bytes) {
  try {
    const bytes = await decompress(c_bytes);
    const content = new TextDecoder().decode(bytes);
    return content;
  } catch (e) {
    console.log("Error decompressing content fragment:", e);
    return null;
  }
}

function isJsonContent(content) {
  try {
    const obj = JSON.parse(content);
    return [true, obj];
  } catch (_) {
    return [false, null];
  }
}

async function getContentFromFragment() {
  const fragment = window.location.hash.slice(1);
  if (fragment.length > 0) {
    const c_bytes = base64url_decode(fragment);
    const data = await decompressFragment(c_bytes);
    if (data !== null) {
      const [is_json, session] = isJsonContent(data);
      if (is_json) {
        try {
          const content = session.sessions[0].content
          editor.getSession().setValue(content);
          return;
        } catch (e) {
          console.log("Bad session data:", e);
        }
      } else {
        editor.getSession().setValue(data);
        return;
      }
    }
  }

  editor.getSession().setValue(default_content);
}


async function* streamAsyncIterator(stream) {
  // Get a lock on the stream
  const reader = stream.getReader();

  try {
    while (true) {
      // Read from the stream
      const { done, value } = await reader.read();
      if (done) return;
      yield value;
    }
  }
  finally {
    reader.releaseLock();
  }
}

function base64url_encode(buffer) {
  return btoa(Array.from(new Uint8Array(buffer), b => String.fromCharCode(b)).join(''))
    .replace(/\+/g, '-')
    .replace(/\//g, '_')
    .replace(/=+$/, '');
}

function base64url_decode(value) {
  const m = value.length % 4;
  return Uint8Array.from(atob(
    value.replace(/-/g, '+')
      .replace(/_/g, '/')
      .padEnd(value.length + (m === 0 ? 0 : 4 - m), '=')
  ), c => c.charCodeAt(0)).buffer
}

async function concatUintArrays(arrays) {
  const blob = new Blob(arrays);
  const buffer = await blob.arrayBuffer();
  return new Uint8Array(buffer);
}

async function compress(bytes) {
  const s = new Blob([bytes]).stream();
  const cs = s.pipeThrough(
    new CompressionStream('deflate-raw')
  );
  const chunks = [];
  for await (const chunk of streamAsyncIterator(cs)) {
    chunks.push(chunk);
  }
  return await concatUintArrays(chunks);
}

async function decompress(bytes) {
  const s = new Blob([bytes]).stream();
  const ds = s.pipeThrough(
    new DecompressionStream('deflate-raw')
  );
  const chunks = [];
  for await (const chunk of streamAsyncIterator(ds)) {
    chunks.push(chunk);
  }
  return await concatUintArrays(chunks);
}
