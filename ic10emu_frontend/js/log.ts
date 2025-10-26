export function logHeaderFormatting(msg: string, style: string, origin: string,): string[] {
  return [
    `%c${msg}%c ${origin}%c`,
    style,
    "color: gray; font-style: italic",
    "color: inherit"
  ];
}

declare var WorkerGlobalScope: {
    prototype: any;
    new(): any;
};

export function getOrigin(framesUp: number = 1) {
  const origin = (new Error()).stack!.split('\n')[framesUp + 1];
  if (typeof WorkerGlobalScope !== 'undefined' && self instanceof WorkerGlobalScope) {
    const workerName = self.name ?? "worker"
    return `(worker: ${workerName})|${origin}`;
  } else {
    return origin;
  }
}

export function error(...args: any[]): void {
  const header = logHeaderFormatting("ERROR", "color: red; background: #444", getOrigin());
  console.error(...header, ...args)
}

export function warn(...args: any[]): void {
  const header = logHeaderFormatting("WARN", "color: orange; background: #444", getOrigin());
  console.warn(...header, ...args)
}

export function info(...args: any[]): void {
  const header = logHeaderFormatting("INFO", "color: whitesmoke; background: #444", getOrigin());
  console.info(...header, ...args)
}

export function debug(...args: any[]): void {
  const header = logHeaderFormatting("DEBUG", "color: lawngreen; background: #444", getOrigin());
  console.debug(...header, ...args)
}

export function trace(...args: any[]): void {
  const header = logHeaderFormatting("TRACE", "color: dodgerblue; background: #444", getOrigin());
  console.log(...header, ...args)
}


