/* eslint-disable @typescript-eslint/no-unsafe-function-type */
/* eslint-disable @typescript-eslint/no-explicit-any */

export default function throttle<T>(func: Function, delay: number) {
  let timeout: number | null = null;
  return function (this: T, ...args: any[]) {
    if (timeout) return;
    func.apply(this, args);
    timeout = setTimeout(() => {
      timeout = null;
    }, delay);
  };
}
