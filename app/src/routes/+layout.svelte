<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import init, { load_bin_hashtables, load_wad_hashtables } from '$lib/pkg/rust';
	import * as hashtables from '$lib/hashtables';

	import * as Sidebar from '$lib/components/ui/sidebar';
	import AppSidebar from '$lib/components/sidebar.svelte';
	import { onMount } from 'svelte';
	import { toast } from 'svelte-sonner';
	import { base } from '$app/paths';
	import { writable, type Writable } from 'svelte/store';
	import { Toaster } from '$lib/components/ui/sonner';

	let { children } = $props();

	const wad_ready = hashtables.wad_ready.set(writable(false));
	const bin_ready = hashtables.bin_ready.set(writable(false));

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

		const load_hashes = (
			func: typeof load_bin_hashtables,
			name: string,
			store: Writable<boolean>
		) => {
			toast.info(`Loading ${name} hashtables...`, {});
			func(`${base}/hashes`).then((count) => {
				if (count <= 0) {
					toast.error(`${name} Hashtables were empty! Could not load any hashes.`);
				} else if (count <= 1000) {
					toast.warning(`Loaded ${count.toLocaleString()} ${name} hashes?`, {});
				} else {
					toast.success(`Loaded ${count.toLocaleString()} ${name} hashes!`, {});
				}
				store.set(true);
			});
		};
		load_hashes(load_wad_hashtables, 'Wad', wad_ready);
		load_hashes(load_bin_hashtables, 'Bin', bin_ready);
	});
</script>

<ModeWatcher />
<Toaster richColors />
<Sidebar.Provider>
	<AppSidebar />
	<main class="flex w-full flex-col p-5">
		<Sidebar.Trigger />
		{@render children?.()}
	</main>
</Sidebar.Provider>
