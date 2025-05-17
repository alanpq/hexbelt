<script lang="ts">
	import {
		Box,
		Download,
		Eye,
		File as FileIcon,
		FileImage,
		Folder,
		LetterText,
		PersonStanding,
		SendToBack,
		Skull,
		Table2,
		Undo2,
		Image as ImageIcon,
		X
	} from '@lucide/svelte';

	import DropZone from '$lib/components/DropZone.svelte';

	import { Button } from '$lib/components/ui/button';
	import * as Sidebar from '$lib/components/ui/sidebar';
	import * as ContextMenu from '$lib/components/ui/context-menu';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import * as Resizable from '$lib/components/ui/resizable';
	import * as Accordion from '$lib/components/ui/accordion';
	import * as Card from '$lib/components/ui/card';
	import * as Tabs from '$lib/components/ui/tabs/index.js';

	import * as context from '$lib/context';
	import { previewableExtensions } from '$lib/consts';

	import type { Component } from 'svelte';
	import { fade } from 'svelte/transition';
	import { toast, Toaster } from 'svelte-sonner';

	import { Bin, Item, type TreeNode } from '$lib/pkg/rust';
	import DropOverlay from '$lib/components/DropOverlay.svelte';
	import { goto } from '$app/navigation';
	import { base } from '$app/paths';
	import { browser } from '$app/environment';
	import { cn } from '$lib/utils';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import UploadButton from '$lib/components/UploadButton.svelte';
	import { openWad } from '$lib/context';
	import { assert, Node, parseColor, type ValueColor } from './process';
	import AccordionItem from '$lib/components/ui/accordion/accordion-item.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import AccordionContent from '$lib/components/ui/accordion/accordion-content.svelte';
	import Checkbox from '$lib/components/ui/checkbox/checkbox.svelte';
	import SmallColor from './SmallColor.svelte';
	import LargeColor from './LargeColor.svelte';
	import { SvelteSet } from 'svelte/reactivity';

	let ctx = context.binsplash.get();
	let root = $derived(ctx.bin?.data.tree);
	$inspect(root);

	let browserOpen = $derived((ctx.bin && root) || ctx.opening);

	const onFiles = async (files: FileList) => {
		context.openBinsplash(ctx, files);
	};

	let characters: {
		[character: string]: {
			[skin: string]: {
				[particle: string]: {
					[emitter: string]: {
						color?: ValueColor;
						birthColor?: ValueColor;
						lingerColor?: ValueColor;
					};
				};
			};
		};
	} = $derived.by(() => {
		if (!root || !ctx.bin) return {};

		try {
			assert(Node.isNamespace(root), 'Invalid root');
			const chars = root.value[1]['Characters'];
			assert(Node.isNamespace(chars), "'Characters' namespace not found");
			const champs: typeof characters = Object.fromEntries(
				Object.values(chars.value[1]).map((character) => {
					assert(Node.isNamespace(character), `Character '${character.value[0]}' not a namespace`);
					const skins = character.value[1]['Skins'];
					assert(Node.isNamespace(skins), "Namespace 'Skins' not found");
					console.log({ skins });
					return [
						character.value[0],
						Object.fromEntries(
							Object.values(skins.value[1]).map((skin) => {
								assert(Node.isNamespace(skin), `Skin '${skin.value[0]}' not a namespace`);
								const particles = skin.value[1]['Particles'];
								assert(Node.isNamespace(particles), "Namespace 'Particles' not found");
								return [
									skin.value[0],
									Object.fromEntries(
										Object.values(particles.value[1]).map((particle) => {
											assert(
												!Node.isNamespace(particle),
												`Particle '${particle.value[0]} is a namespace`
											);
											const value = ctx.bin?.data.objects[particle.value[1]];
											assert(value, 'Particle object not found');
											const children = value.children.filter(
												(child) => child.name === 'complexEmitterDefinitionData'
											);
											assert(
												children.length == 1,
												'>1 complexEmitterDefinitionData found in one particle definition!'
											);
											const emitters = children[0].children;
											console.log({ emitters });
											return [
												particle.value[0],
												Object.fromEntries(
													emitters.map((emitter) => {
														const nameEntry = emitter.children.find(
															(c) => c.name == 'emitterName'
														)?.value;
														assert(nameEntry !== undefined, "'emitterName' not found");
														assert(
															nameEntry.kind === 'PropertyJSValue' &&
																nameEntry.value.kind === 'String',
															"'emitterName' not a string"
														);
														const name = nameEntry.value.value;
														const children = Object.fromEntries(
															emitter.children.map((child) => [child.name, child])
														);
														return [
															name,
															{
																color: children.color && parseColor(children.color),
																birthColor: children.birthColor && parseColor(children.birthColor),
																lingerColor:
																	children.lingerColor && parseColor(children.lingerColor)
															}
														];
													})
												)
											];
										})
									)
								];
							})
						)
					];
				})
			);
			console.log({ champs });
			return champs;
		} catch (e) {
			console.log('Could not find particle defs:', e);
			toast.error(`Could not find particle definitions:\n${e}`);
		}
		return {};
	});
</script>

{#if !browserOpen}
	<main class="flex w-full flex-col p-5" out:fade={{ duration: 100 }}>
		<Sidebar.Trigger />
		<div class="flex flex-grow">
			<DropZone class="m-5 flex-grow" {onFiles}>
				<h2>No file open.</h2>
				<p class="text-sm text-muted-foreground">Drag and drop a file or</p>
				<UploadButton {onFiles} />
			</DropZone>
		</div>
	</main>
{:else}
	{@const chars = Object.entries(characters)}
	<main class="flex w-full flex-col p-5" in:fade={{ delay: 100, duration: 100 }}>
		<DropOverlay {onFiles} class="flex">
			<Tabs.Root value={chars[0]?.[0]} class="">
				<Tabs.List
					class="grid w-full"
					style={`grid-template-columns: repeat(${chars.length},1fr);`}
				>
					{#each chars as [character, _]}
						<Tabs.Trigger value={character}>{character}</Tabs.Trigger>
					{/each}
				</Tabs.List>
				{#each chars as [character, skins_obj]}
					{@const skins = Object.entries(skins_obj)}
					<Tabs.Content value={character}>
						<Tabs.Root value={skins[0]?.[0]}>
							<Tabs.List
								class="grid w-full"
								style={`grid-template-columns: repeat(${skins.length},1fr);`}
							>
								{#each skins as [skin, _]}
									<Tabs.Trigger value={skin}>{skin}</Tabs.Trigger>
								{/each}
							</Tabs.List>
							{#each skins as [skin, particles_obj]}
								{@const particles = Object.entries(particles_obj)}
								<Tabs.Content value={skin}>
									<Accordion.Root type="multiple" value={particles.map((p) => p[0])}>
										{#each particles as [particle, emitters_obj]}
											{@const emitters = Object.entries(emitters_obj)}
											{@const selectedKey = `${character}/${skin}/${particle}`}
											{@const emitterSelected = ctx.selected.get(selectedKey)?.size ?? 0}
											<Accordion.Item value={particle}>
												<Accordion.Trigger class="h-6 py-5">
													<span class="flex flex-row gap-2">
														<Checkbox
															checked={emitterSelected == emitters.length}
															indeterminate={emitterSelected > 0 &&
																emitterSelected < emitters.length}
															onCheckedChange={(val) => {
																const set = ctx.selected.get(selectedKey) ?? new SvelteSet();
																for (const [emitter, _] of emitters) {
																	val ? set.add(emitter) : set.delete(emitter);
																}
																ctx.selected.set(selectedKey, set);
															}}
															onclick={(e) => {
																e.stopPropagation();
															}}
														/>
														{particle}
													</span>
												</Accordion.Trigger>
												<Accordion.Content>
													<ul
														class="grid grid-cols-[max-content,minmax(10ch,1fr),repeat(4,minmax(1em,5em)),minmax(4em,15em)] gap-1 pl-2"
													>
														{#each emitters as [name, emitter]}
															<li class="col-span-full grid grid-cols-subgrid place-items-center">
																<Checkbox
																	checked={ctx.selected.get(selectedKey)?.has(name)}
																	onCheckedChange={(val) => {
																		const set = ctx.selected.get(selectedKey) ?? new SvelteSet();
																		val ? set.add(name) : set.delete(name);
																		ctx.selected.set(selectedKey, set);
																		console.log({ selected: ctx.selected });
																	}}
																/>
																<span class="place-self-stretch truncate pl-2">
																	{name}
																</span>
																<SmallColor />
																<SmallColor />
																<SmallColor />
																<SmallColor color={emitter.birthColor} />
																<LargeColor color={emitter.color} />
															</li>
														{/each}
													</ul>
												</Accordion.Content>
											</Accordion.Item>
										{/each}
									</Accordion.Root>
								</Tabs.Content>
							{/each}
						</Tabs.Root>
					</Tabs.Content>
				{/each}
			</Tabs.Root>
		</DropOverlay>
	</main>
{/if}
