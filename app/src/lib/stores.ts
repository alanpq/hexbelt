import { getContext } from "svelte"
import type { Writable } from "svelte/store";
import type { WadTree } from "rust";

export const wad_hashtables = (): Writable<boolean> => {
  return getContext("wad_hashtables");
}
export const bin_hashtables = (): Writable<boolean> => {
  return getContext("bin_hashtables");
}

export const bin_src = (): Writable<Uint8Array | null> => {
  return getContext("bin_src");
}

export const wad = (): Writable<WadTree | null> => {
  return getContext("wad");
}
export const wad_path = (): Writable<number[]> => {
  return getContext("wad_path");
}