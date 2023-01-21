<script>
  import {onMount} from 'svelte'
  import SvelteMarkdown from 'svelte-markdown'
  import CodeRenderer from '../../renderers/Code.svelte';
  import TheBanner from '../../lib/TheBanner.svelte';
  import HighlightCard from '../../lib/DisplayCard.svelte';
  import FlipCard from '../../lib/FlipCard.svelte';
  
  import 'highlight.js/styles/base16/monokai.css';
  
  let selected = -1;

  let source = `
  # Brooks Palin, Programmer

  **This is a bold sentence.**

  There is a programmer of legend

  \`\`\`js
  function test() {
    console.log("HEllo World!")
  }
  \`\`\`
  `

  let blogs = [];

  onMount(async () => {
    let res = await fetch('/blog/batch?offset=' + 0 + "&count=" + 4);
    let data = await res.json();
    blogs = data;
  })
</script>

<div class="app-wrapper">
  <TheBanner current_page={selected} on:tab-clicked={(event) => {selected = event.detail.value}}></TheBanner>
  <div class="content-wrapper">
    
    <SvelteMarkdown renderers={{code : CodeRenderer}} {source}></SvelteMarkdown>
    <h2>Blogs</h2>
    <div class="gridlayout">
      {#each blogs.reverse() as blog, index}
        <FlipCard 
        color={1} 
        image_url={blog.path + '/' + blog.pic}
        title={blog.title}
        height="400px"
        link={"/blog?id=" + index}
        />
      {/each}
    </div>
  </div>
</div>

<style>
  .gridlayout {
    display: grid;
    grid-template-columns:1fr 1fr;
    grid-gap: 25px;
    width: 100%;
    flex-basis: 45%;
    align-items: stretch;
  }
</style>
