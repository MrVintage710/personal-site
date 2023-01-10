<script>
  import {onMount} from 'svelte'
  import SvelteMarkdown from 'svelte-markdown';
  import CodeRenderer from '../../renderers/Code.svelte';
  import ImageRenderer from '../../renderers/Image.svelte';
  import 'highlight.js/styles/base16/monokai.css';

  const urlParams = new URLSearchParams(window.location.search);
  const id = Number(urlParams.get('id'));

  let blog = {
    content: "",
    meta : {
      title : ""
    }
  };

  onMount(async () => {
    let res = await fetch('/blog/single?id=' + id);
    let data = await res.json();
    blog = data
  })

  $: title = blog.meta.title;
  $: content = blog.content;
</script>

<div class="app-wrapper">
  <div class="content-wrapper">
    <h1>{title}</h1>
    <SvelteMarkdown renderers={{code : CodeRenderer, image: ImageRenderer}} source = {content}></SvelteMarkdown>
  </div>
</div>

<style>
  
</style>
