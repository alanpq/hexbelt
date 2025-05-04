import { Context } from 'runed';
import type { Writable } from 'svelte/store';

export const wad_ready = new Context<Writable<boolean>>('wad_tables_ready');
export const bin_ready = new Context<Writable<boolean>>('bin_tables_ready');
