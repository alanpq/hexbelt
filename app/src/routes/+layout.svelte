<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import init, { load_bin_hashtables, load_wad_hashtables } from '$lib/pkg/rust';

	import * as Sidebar from '$lib/components/ui/sidebar';
	import AppSidebar from '$lib/components/sidebar.svelte';
	import { onMount } from 'svelte';
	import { toast } from 'svelte-sonner';
	import { base } from '$app/paths';
	import { Toaster } from '$lib/components/ui/sonner';
	import * as context from '$lib/context';
	import { SvelteSet } from 'svelte/reactivity';

	let { children } = $props();

	let wad: ReturnType<typeof context.wad.get> = $state({ wad: null, path: [], selected: null });
	context.wad.set(wad);
	let bin: ReturnType<typeof context.bin.get> = $state({ bin: null, expanded: new SvelteSet() });
	context.bin.set(bin);

	const hashtables = $state({
		bin: false,
		wad: false
	});
	context.hashtables_ready.set(hashtables);

	onMount(async () => {
		try {
			await init();
		} catch (e) {
			toast.error('Failed to load WASM module!', {
				important: true,
				description: `${e}`
			});
			console.error('Failed to load WASM module:', e);
		}

		const load_hashes = async (func: typeof load_bin_hashtables, name: string) => {
			toast.info(`Loading ${name} hashtables...`, {});
			await func(`${base}/hashes`).then((count) => {
				if (count <= 0) {
					toast.error(`${name} Hashtables were empty! Could not load any hashes.`);
				} else if (count <= 1000) {
					toast.warning(`Loaded ${count.toLocaleString()} ${name} hashes?`, {});
				} else {
					toast.success(`Loaded ${count.toLocaleString()} ${name} hashes!`, {});
				}
			});
		};
		load_hashes(load_wad_hashtables, 'Wad').then(() => (hashtables.wad = true));
		load_hashes(load_bin_hashtables, 'Bin').then(() => (hashtables.bin = true));
	});
</script>

<ModeWatcher />
<Toaster richColors />
<Sidebar.Provider>
	<AppSidebar />
	{@render children?.()}
</Sidebar.Provider>
