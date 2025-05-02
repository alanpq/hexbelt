<script lang="ts">
  import { preventDefault } from 'svelte/legacy';

  

  

  

  

  
  interface Props {
    /** DOM element of the label wrapper */
    labelElement: HTMLLabelElement;
    /** hex color */
    hex: string | undefined;
    /** input label */
    label: string;
    /** input name, useful in a native form */
    name?: string | undefined;
    /* svelte-ignore unused-export-let /** indicator of the popup state */
    isOpen: boolean;
  }

  let {
    labelElement = $bindable(),
    hex,
    label,
    name = undefined,
    isOpen
  }: Props = $props();

  function noop() {
    /* prevent browser color picker from opening unless javascript is broken */
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions, a11y_click_events_have_key_events -->
<label
  bind:this={labelElement}
  onclick={preventDefault(noop)}
  onmousedown={preventDefault(noop)}
>
  <div class="relative flex items-center justify-center w-20">
    <input
      type="color"
      {name}
      value={hex}
      onclick={preventDefault(noop)}
      onmousedown={preventDefault(noop)}
      aria-haspopup="dialog"
    />
    <div class="alpha w-20 h-4"></div>
    <div class="color w-20 h-4" style:background={hex}></div>
  </div>
</label>

<style>
  label {
    cursor: pointer;
    user-select: none;
  }

  input {
    margin: 0;
    padding: 0;
    border: none;
    width: 1px;
    height: 1px;
    flex-shrink: 0;
    opacity: 0;
  }

  .alpha {
    /* clip-path: circle(50%); */
    background: var(--alpha-grid-bg);
  }

  .alpha,
  .color {
    position: absolute;
    /* border-radius: 50%; */
    user-select: none;
  }

  input:focus-visible ~ .color {
    outline: 2px solid var(--focus-color, red);
    outline-offset: 2px;
  }
</style>
