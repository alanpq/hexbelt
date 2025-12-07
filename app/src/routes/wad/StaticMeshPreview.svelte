<script lang="ts">
	import { Canvas } from '@threlte/core';
	import { Environment, OrbitControls } from '@threlte/extras';
	import { T } from '@threlte/core';
	import { BufferGeometry } from 'three';

	import * as context from '$lib/context';
	import type { Item } from '$lib/pkg/rust';

	let ctx = context.wad.get();
	let { item }: { item: Item } = $props();
	let data = $derived(ctx.wad && ctx.wad.load_chunk_data(item.id));

	let geometry = new BufferGeometry();
</script>

<Canvas>
	<T.PerspectiveCamera makeDefault position={10}>
		<OrbitControls autoRotate autoRotateSpeed={0.5} />
	</T.PerspectiveCamera>
	<T.Mesh {geometry}>
		<T.MeshBasicMaterial />
	</T.Mesh>
</Canvas>
