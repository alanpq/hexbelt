<script lang="ts">
  /** DOM element of the label wrapper */
  export let labelElement: HTMLLabelElement;

  /** hex color */
  export let hex: string | undefined;

  /** input label */
  export let label: string;

  /** input name, useful in a native form */
  export let name: string | undefined = undefined;

  /* svelte-ignore unused-export-let /** indicator of the popup state */
  export let isOpen: boolean;

  function noop() {
    /* prevent browser color picker from opening unless javascript is broken */
  }
</script>

<!-- svelte-ignore a11y-no-noninteractive-element-interactions a11y-click-events-have-key-events -->
<label
  bind:this={labelElement}
  on:click|preventDefault={noop}
  on:mousedown|preventDefault={noop}
>
  <div class="relative flex items-center justify-center w-20">
    <input
      type="color"
      {name}
      value={hex}
      on:click|preventDefault={noop}
      on:mousedown|preventDefault={noop}
      aria-haspopup="dialog"
    />
    <div class="alpha w-20 h-4" />
    <div class="color w-20 h-4" style:background={hex} />
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
