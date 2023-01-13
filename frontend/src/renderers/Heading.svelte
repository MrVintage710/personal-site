<script>
    import { getContext } from 'svelte'
    import { key } from 'svelte-markdown/src/context'
    export let depth
    export let raw
    export let text
    const { slug, getOptions } = getContext(key)
    const options = getOptions()
    $: id = options.headerIds
      ? options.headerPrefix + slug(text)
      : undefined
  </script>
  
  {#if depth === 1}
    <h1 {id} class="heading-1"><slot></slot></h1>
  {:else if depth === 2}
    <h2 {id} class="heading-2"><slot></slot></h2>
  {:else if depth === 3}
    <h3 {id} class="heading-3"><slot></slot></h3>
  {:else if depth === 4}
    <h4 {id} class="heading-4"><slot></slot></h4>
  {:else if depth === 5}
    <h5 {id} class="heading-5"><slot></slot></h5>
  {:else if depth === 6}
    <h6 {id} class="heading-6"><slot></slot></h6>
  {:else}
    {raw}
  {/if}