import type { BinEntryValue } from "rust";
import type { SvelteComponent } from "svelte";
import Unknown from "./Unknown.svelte";
import type { Constructor } from "svelte-render";
import JsValue from "./JSValue.svelte";
import Object from "./Object.svelte";
import Struct from "./Struct.svelte";
import Container from "./Container.svelte";
import Optional from "./Optional.svelte";
import Map from "./Map.svelte";
import MapEntry from "./MapEntry.svelte";
import Empty from "./Empty.svelte";

export const types: {
	[K in BinEntryValue["kind"]]: Constructor<SvelteComponent>;
} = {
	Object: Object,
	Namespace: Empty,
	PropertyJSValue: JsValue,
	PropertyNone: Unknown,
	PropertyContainer: Container,
	PropertyUnorderedContainer: Container,
	PropertyMap: Map,
	PropertyMapEntry: MapEntry,
	PropertyStruct: Struct,
	PropertyEmbedded: Struct,
	PropertyOptional: Optional,
};

export const get_type = (k: unknown) => {
	return types[k as BinEntryValue["kind"]] ?? Unknown;
};
