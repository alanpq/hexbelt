<script lang="ts">
	import { PackageOpen, House, Table2, Sun, Moon, LoaderCircle, Paintbrush } from '@lucide/svelte';
	import { toggleMode } from 'mode-watcher';

	import { page } from '$app/state';

	import * as Sidebar from '$lib/components/ui/sidebar';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { hashtables_ready } from '$lib/context';
	import { fade } from 'svelte/transition';
	import type { Component } from 'svelte';
	import { base } from '$app/paths';

	const hashtables = hashtables_ready.get();

	const items: {
		title: string;
		url: string;
		icon: Component;
		spinner?: keyof typeof hashtables;
	}[] = [
		{
			title: 'Home',
			url: '/',
			icon: House
		},
		{
			title: 'Wad Walker',
			url: '/wad',
			icon: PackageOpen,
			spinner: 'wad'
		},
		{
			title: 'Bin Explorer',
			url: '/bin',
			icon: Table2,
			spinner: 'bin'
		},
		{ title: 'Bin Splash', url: '/bin-splash', icon: Paintbrush, spinner: 'bin' }
	];
</script>

<Sidebar.Root collapsible="icon">
	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.GroupLabel class="flex pr-0">
				<span class="flex-grow">hexbelt</span>
				<Tooltip.Provider>
					<Tooltip.Root>
						<Tooltip.Trigger>
							{#snippet child({ props })}
								<button
									{...props}
									class="-mt-0.5 flex p-1 hover:text-foreground"
									onclick={toggleMode}
								>
									<Sun
										class="size-[1.2em] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
									/>
									<Moon
										class="absolute size-[1.2em] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
									/>
								</button>
							{/snippet}
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>Toggle theme</p>
						</Tooltip.Content>
					</Tooltip.Root>
				</Tooltip.Provider>
			</Sidebar.GroupLabel>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					{#each items as item (item.title)}
						<Sidebar.MenuItem>
							<Sidebar.MenuButton isActive={page.url.pathname === item.url}>
								{#snippet child({ props })}
									<a href={base + item.url} {...props}>
										<item.icon />
										<span class="flex-grow">{item.title}</span>
										{#if item.spinner !== undefined && !hashtables[item.spinner]}
											<span transition:fade>
												<LoaderCircle class="size-4 animate-spin" />
											</span>
										{/if}
									</a>
								{/snippet}
								{#snippet tooltipContent()}
									{item.title}
								{/snippet}
							</Sidebar.MenuButton>
						</Sidebar.MenuItem>
					{/each}
				</Sidebar.Menu>
			</Sidebar.GroupContent>
		</Sidebar.Group>
	</Sidebar.Content>
</Sidebar.Root>
