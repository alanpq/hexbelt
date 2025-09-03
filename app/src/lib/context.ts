import { Context } from 'runed';
import { type WadTree, type Bin, open_wad, open_bin, type TreeEdits } from '$lib/pkg/rust';
import type { SvelteMap, SvelteSet } from 'svelte/reactivity';
import { toast } from 'svelte-sonner';

export const hashtables_ready = new Context<{ wad: boolean; bin: boolean }>('hashtables_ready');

export const wad = new Context<{
	wad: WadTree | null;
	path: number[];
	selected: number | null;
	opening: boolean | null;
}>('wad');
export type WadContext = ReturnType<typeof wad.get>;

export const bin = new Context<{
	bin: Bin | null;
	expanded: SvelteSet<string>;
	opening: boolean | null;
}>('bin');
export type BinContext = ReturnType<typeof bin.get>;

export const binsplash = new Context<{
	bin: Bin | null;
	edit: TreeEdits;
	selected: SvelteMap<string, SvelteSet<string>>;
	opening: boolean | null;
}>('binsplash');
export type BinSplashContext = ReturnType<typeof binsplash.get>;

export const openWad = async (ctx: WadContext, files: FileList) => {
	ctx.opening = true;
	try {
		ctx.wad = null;
		ctx.wad = await open_wad(files[0]);
		if (!ctx.wad) throw new Error('failed to open wad');
		console.log(Array.from(ctx.wad.children).map((i) => ctx.wad?.get(i)));
		ctx.path = [];
	} catch (e) {
		console.error('failed to open wad: ', e);
		toast.error(`Failed to open Wad! ${e}`);
	}
	ctx.opening = false;
};

export const openBin = async (ctx: BinContext, files: FileList) => {
	ctx.opening = true;
	ctx.bin = null;
	ctx.bin = await open_bin(files[0]);
	ctx.opening = false;
};

export const openBinsplash = async (ctx: BinSplashContext, files: FileList) => {
	ctx.opening = true;
	ctx.bin = null;
	ctx.bin = await open_bin(files[0]);
	ctx.opening = false;
};
