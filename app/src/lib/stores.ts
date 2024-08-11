import { getContext } from "svelte"
import type { Writable } from "svelte/store";

export const hashtables = (): Writable<boolean> => {
  return getContext("hashtables");
}

export const bin_src = (): Writable<Uint8Array | null> => {
  return getContext("bin_src");
}