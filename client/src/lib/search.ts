import { Payload } from './payload';

export const search = async (query: string): Promise<Payload> => {
  return await (
    await fetch('/search?query=' + encodeURIComponent(query))
  ).json();
};
