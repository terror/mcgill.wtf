export const debounce = (f: (v: string) => void, delay: number) => {
  let lastTimeout: number = 0;
  return (value: string) => {
    if (lastTimeout) clearTimeout(lastTimeout);
    lastTimeout = setTimeout(() => {
      f(value);
    }, delay);
  };
};
