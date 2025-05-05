import { Context } from 'runed';
import type { WadTree, Bin } from '$lib/pkg/rust';
import type { SvelteSet } from 'svelte/reactivity';

export const hashtables_ready = new Context<{ wad: boolean; bin: boolean }>('hashtables_ready');

export const wad = new Context<{
	wad: WadTree | null;
	path: number[];
	selected: number | null;
}>('wad');

export const bin = new Context<{
	bin: Bin | null;
	expanded: SvelteSet<string>;
}>('bin');
