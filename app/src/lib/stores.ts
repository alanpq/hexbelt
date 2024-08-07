import { getContext } from "svelte"
import type { Writable } from "svelte/store";

export const hashtables = (): Writable<boolean> => {
  return getContext("hashtables");
}