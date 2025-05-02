<script lang="ts">
	import { PackageOpen, House, Table2, Sun, Moon } from '@lucide/svelte';

	import * as Sidebar from '$lib/components/ui/sidebar';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { page } from '$app/state';
	import Lightswitch from './Lightswitch.svelte';
	import Button from './ui/button/button.svelte';
	import { toggleMode } from 'mode-watcher';

	const items = [
		{
			title: 'Home',
			url: '/',
			icon: House
		},
		{
			title: 'Wad Walker',
			url: '/wad',
			icon: PackageOpen
		},
		{
			title: 'Bin Explorer',
			url: '/bin',
			icon: Table2
		}
	];
</script>

<Sidebar.Root collapsible="icon">
	<Sidebar.Content>
		<Sidebar.Group class="pt-0">
			<Sidebar.GroupLabel class="flex pr-0">
				<span class="flex-grow">hexbelt</span>
				<Tooltip.Provider>
					<Tooltip.Root>
						<Tooltip.Trigger>
							{#snippet child({ props })}
								<button {...props} class="flex p-1 hover:text-foreground" onclick={toggleMode}>
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
									<a href={item.url} {...props}>
										<item.icon />
										<span>{item.title}</span>
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
