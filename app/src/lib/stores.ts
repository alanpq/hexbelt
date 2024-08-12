import { getContext } from "svelte"
import type { Writable } from "svelte/store";

export const wad_hashtables = (): Writable<boolean> => {
  return getContext("wad_hashtables");
}
export const bin_hashtables = (): Writable<boolean> => {
  return getContext("bin_hashtables");
}

export const bin_src = (): Writable<Uint8Array | null> => {
  return getContext("bin_src");
}