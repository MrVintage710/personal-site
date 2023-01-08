<script>
  import SvelteMarkdown from 'svelte-markdown'
  import CodeRenderer from './renderers/Code.svelte';
  import TheBanner from './lib/TheBanner.svelte';
  import HighlightCard from './lib/HighlightCard.svelte';
  import FlipCard from './lib/FlipCard.svelte';
  
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

</script>

<div class="app-wrapper">
  <TheBanner current_page={selected} on:tab-clicked={(event) => {selected = event.detail.value}}></TheBanner>
  <div class="content-wrapper">
    
    <SvelteMarkdown renderers={{code : CodeRenderer}} {source}></SvelteMarkdown>
    <h2>Blogs</h2>
    <div class="gridlayout">
        <FlipCard 
        color={1} 
        image_url={"https://static01.nyt.com/images/2022/04/04/multimedia/15ai-nocode/15ai-nocode-superJumbo.gif?quality=75&auto=webp"}
        title={"Programing a Game Engine in Rust From Scratch - A Cautionary Tale"},
        height="400px"
        />
        <FlipCard 
        color={2} 
        image_url={"https://static01.nyt.com/images/2022/04/04/multimedia/15ai-nocode/15ai-nocode-superJumbo.gif?quality=75&auto=webp"}
        title={"Programing a Game Engine in Rust From Scratch - A Cautionary Tale"},
        height="400px"
        />
        <FlipCard 
        color={3} 
        image_url={"https://static01.nyt.com/images/2022/04/04/multimedia/15ai-nocode/15ai-nocode-superJumbo.gif?quality=75&auto=webp"}
        title={"Programing a Game Engine in Rust From Scratch - A Cautionary Tale"},
        height="400px"
        />
        <FlipCard 
        color={4} 
        image_url={"https://static01.nyt.com/images/2022/04/04/multimedia/15ai-nocode/15ai-nocode-superJumbo.gif?quality=75&auto=webp"}
        title={"Programing a Game Engine in Rust From Scratch - A Cautionary Tale"},
        height="400px"
        />
    </div>
  </div>
</div>

<style>
  .app-wrapper {
    overflow-x: hidden;
    max-width: 100vw;
    position: relative;
  }

  .content-wrapper  {
    max-width: 1225px;
    padding: 0px 10px 0px 10px;
    transform: translate(calc(50vw - 50%));
  }

  .gridlayout {
    display: grid;
    grid-template-columns:1fr 1fr;
    grid-gap: 25px;
    width: 100%;
    flex-basis: 45%;
    align-items: stretch;
  }
</style>
