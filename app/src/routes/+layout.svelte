<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';

	import * as Sidebar from '$lib/components/ui/sidebar';
	import AppSidebar from '$lib/components/sidebar.svelte';

	let { children } = $props();
</script>

<ModeWatcher />
<Sidebar.Provider>
	<AppSidebar />
	<main class="flex w-full flex-col p-5">
		<Sidebar.Trigger class="mb-5" />
		{@render children?.()}
	</main>
</Sidebar.Provider>
