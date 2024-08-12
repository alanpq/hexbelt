import type { BinEntryValue } from "rust";
import type { SvelteComponent } from "svelte";
import Unknown from "./Unknown.svelte";
import type { Constructor } from "svelte-render";
import JsValue from "./JSValue.svelte";
import Object from "./Object.svelte";
import Struct from "./Struct.svelte";
import Container from "./Container.svelte";
import Optional from "./Optional.svelte";

export const types: { [K in BinEntryValue["kind"]]: Constructor<SvelteComponent> } = {
  Object: Object,
  PropertyJSValue: JsValue,
  PropertyNone: Unknown,
  PropertyContainer: Container,
  PropertyUnorderedContainer: Container,
  PropertyMap: Unknown,
  PropertyMapEntry: Unknown,
  PropertyStruct: Struct,
  PropertyEmbedded: Struct,
  PropertyOptional: Optional,
};

export const get_type = (k: unknown) => {
  return types[k as BinEntryValue["kind"]] ?? Unknown;
}