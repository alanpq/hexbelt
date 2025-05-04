import { Context } from 'runed';
import type { WadTree } from '$lib/pkg/rust';

export const hashtables_ready = new Context<{ wad: boolean; bin: boolean }>('hashtables_ready');

export const wad = new Context<{
	wad: WadTree | null;
	path: number[];
	selected: number | null;
}>('wad');
