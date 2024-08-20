function jsonMatchingBrace(char: string): string {
  switch (char) {
    case "[":
      return "]";
    case "]":
      return "[";
    case "{":
      return "}";
    case "}":
      return "{";
    default:
      return char;
  }
}

function readJsonContextBack(ctx: string, maxLen: number, stringCanEnd: boolean): string {
  const tokenContexts: string[] = [];
  let inStr: boolean = false;
  let foundOpen: boolean = false;
  let ctxEnd: number = 0;
  let lastChar: string | null = null;
  let lastNonWhitespaceChar: string | null = null;
  let countStringOpens: number = 0;
  let countObjectsFound: number = 0;
  const chars = ctx.split("");
  for (let i = chars.length; i >= 0; i--) {
    const c = chars[i];
    if (c === ":" && inStr && countStringOpens === 1 && lastNonWhitespaceChar === '"') {
      inStr = false;
      tokenContexts.pop();
    }
    if (c === "\\" && !inStr && lastChar === '"') {
      if (lastChar != null) {
        tokenContexts.push(lastChar);
      }
      foundOpen = false;
      inStr = true;
      continue;
    }
    if (c === '"') {
      if (inStr && (tokenContexts.length > 0 ? tokenContexts[tokenContexts.length - 1] : null) === c) {
        inStr = false;
        if (stringCanEnd) {
          foundOpen = true;
        }
        tokenContexts.pop();
      } else {
        inStr = true;
        countStringOpens += 1;
        tokenContexts.push(c);
      }
    }
    if ((c === "]" || c === "}") && !inStr) {
      tokenContexts.push(c);
    }
    if (
      (c === "[" || c === "{")
      && !inStr
      && (tokenContexts.length > 0 ? tokenContexts[tokenContexts.length - 1] : null) === jsonMatchingBrace(c)
    ) {
      tokenContexts.pop();
      foundOpen = true;
    }
    if (
      (c === ",")
      && !inStr
      && (lastNonWhitespaceChar === "[" || lastNonWhitespaceChar === "{" || lastNonWhitespaceChar === '"')
    ) {
      foundOpen = true;
    }

    lastChar = c;
    if (!(' \t\n\r\v'.indexOf(c) > -1)) {
      lastNonWhitespaceChar = c;
    }
    if (foundOpen && !tokenContexts.length) {
      countObjectsFound += 1;
    }
    if (countObjectsFound >= 2) {
      ctxEnd = i;
      break;
    }
  }
  if (ctxEnd > 0) {
    ctx = ctx.substring(ctxEnd);
  }
  if (maxLen > 0 && ctx.length > maxLen) {
    ctx = "... " + ctx.substring(maxLen);
  }
  return ctx;
}

function readJsonContextForward(ctx: string, maxLen: number, stringCanEnd: boolean): string {
  const tokenContexts: string[] = [];
  let inStr: boolean = false;
  let foundClose: boolean = false;
  let ctxEnd: number = 0;
  let lastChar: string | null = null;
  let lastNonWhitespaceChar: string | null = null;
  let countStringOpens: number = 0;
  let countObjectsFound: number = 0;
  const chars = ctx.split("");
  for (let i = 0; i < chars.length; i++) {
    const c = chars[i];
    if (c === ":" && inStr && countStringOpens === 1 && lastNonWhitespaceChar === '"') {
      inStr = false;
      tokenContexts.pop();
    }
    if (c === "\\" && !inStr && lastChar === "'") {
      if (lastChar != null) {
        tokenContexts.push(lastChar);
      }
      foundClose = false;
      inStr = true;
      continue;
    }
    if (c === '"') {
      if (
        inStr
        && (tokenContexts.length > 0 ? tokenContexts[tokenContexts.length - 1] : null) === c
      ) {
        inStr = false;
        if (stringCanEnd) {
          foundClose = true;
        }
        tokenContexts.pop();
      } else {
        inStr = true;
        countStringOpens += 1;
        tokenContexts.push(c);
      }
    }
    if (
      (c === "]" || c === "}")
      && !inStr
    ) {
      tokenContexts.pop();
      foundClose = true;
    }
    if (
      (c === "[" || c === "{")
      && !inStr
      && (tokenContexts.length > 0 ? tokenContexts[tokenContexts.length - 1] : null) === jsonMatchingBrace(c)
    ) {
      tokenContexts.push(c);
    }
    if (
      (c === ",")
      && !inStr
      && (lastNonWhitespaceChar === "]" || lastNonWhitespaceChar === "}" || lastNonWhitespaceChar === '"')
    ) {
      foundClose = true;
    }

    lastChar = c;
    if (!(' \t\n\r\v'.indexOf(c) > -1)) {
      lastNonWhitespaceChar = c;
    }
    if (foundClose && !tokenContexts.length) {
      countObjectsFound += 1;
    }
    if (countObjectsFound >= 2) {
      ctxEnd = i;
      break;
    }
  }
  if (ctxEnd > 0) {
    ctx = ctx.substring(0, ctxEnd);
  }
  if (maxLen > 0 && ctx.length > maxLen) {
    ctx = ctx.substring(0, maxLen) + " ...";
  }
  return ctx
}

export function getJsonContext(errLine: number, errColumn: number, body: string, maxLen: number): string {
  if (!body.length) {
    return body;
  }

  const stringCanEnd = body[0] !== '[' && body[0] !== "{";

  const lineOffset = (body.split("").map((c, i) => [c, i]).filter(([c, _]) => c === "\n")[errLine - 1] as [string, number] ?? ["", 0] as [string, number])[1];

  const preLine = body.substring(0, lineOffset);
  const ctxLine = body.substring(lineOffset);

  const ctxBefore = preLine + ctxLine.substring(0, errColumn);
  const ctxAfter = ctxLine.substring(errColumn);

  return readJsonContextBack(ctxBefore, maxLen, stringCanEnd) + "<~~" + readJsonContextForward(ctxAfter, maxLen, stringCanEnd);
}
