<script lang="ts">
  import { cn } from "$lib/utils";

  import ColorPicker from "svelte-awesome-color-picker";
  import ColorWrapper from "$lib/components/ColorWrapper.svelte";
  import ColorInput from "$lib/components/ColorInput.svelte";

  interface Props {
    color: [number, number, number, number];
  }

  let { color }: Props = $props();

  // $: byteColor = color.map((c: number) => c * 255);
  let c = $derived({
    r: color[0],
    g: color[1],
    b: color[2],
    a: color[3] / 255,
  });
</script>

<!-- {JSON.stringify(color)} -->
<!-- <span class="w-[10ch] h-4" style="background:{c.toRgbString()};" /> -->
<ColorPicker
  rgb={c}
  components={{ wrapper: ColorWrapper, input: ColorInput }}
/>
