<script>
  import {onMount} from 'svelte'
  import SvelteMarkdown from 'svelte-markdown';
  import CodeRenderer from '../../renderers/Code.svelte';
  import ImageRenderer from '../../renderers/Image.svelte';
  import HeadingRender from '../../renderers/Heading.svelte';
  import Header from '../../lib/Header.svelte';
  import blogtree from '../../assets/blogtree';
  import {current_path} from "../../lib/current_path"
  import 'highlight.js/styles/base16/monokai.css';

  const urlParams = new URLSearchParams(window.location.search);
  const id = Number(urlParams.get('id'));

  let blog = {
    content: "",
    meta : {
      title : "",
      path : "",
      pic : "",
      author : "",
    }
  };

  onMount(async () => {
    let res = await fetch('/blog/single?id=' + id);
    let data = await res.json();
    blog = data
    current_path.set(blog.meta.path)

    blogtree("blog-content")
    
  })

  $: title = blog.meta.title;
  $: content = blog.content;
  $: path = blog.meta.path;
  $: pic = blog.meta.pic;
  $: author = blog.meta.author;
</script>

<div class="app-wrapper">
  <Header></Header>
  <div class="content-wrapper">
    <div class="header-wrapper">
      <div class="header-title">
        <h1>{title}</h1>
        <p>{author}</p>
      </div>
      <img class="header-image" src="{"/assets/" + path + "/" + pic}" alt="" srcset="">
    </div>
    <div id="blog-content">
      <SvelteMarkdown renderers={{code : CodeRenderer, image: ImageRenderer, heading : HeadingRender}} source = {content}></SvelteMarkdown>
    </div>
  </div>
</div>

<style>
  @media (min-width: 2440px) {
    :root {
      --title-font-size: 1.4em;
      --title-font-line-height: 1.7em;
    }
  }

  @media (min-width: 1008px) and (max-width: 2439px) {
    :root {
      --title-font-size: 1.4em;
      --title-font-line-height: 1.7em;
    }
  }

  @media (max-width: 1007px) and (min-width: 641px) {
    :root {
      --title-font-size: 1.1em;
      --title-font-line-height: 1.7em;
    }
  }

  @media (max-width: 640px) {
    :root {
      --title-font-size: 0.9em;
      --title-font-line-height: 1.7em;
    }
  }

  .header-wrapper {
    position: relative;
    margin: 15px 0px;
  }

  .header-image {
    border-radius: 15px;
  }

  .header-title {
    position: absolute;
    padding: 5px 15px;
    backdrop-filter: blur(8px);
    background-color: rgba(0, 0, 0, 0.2);
    font-size: var(--title-font-size);
    margin: 30px;
    line-height: var(--title-font-line-height);
    max-width: 50%;
    max-height: 85%;
    text-align: center;
    border-radius: 15px;
  }

  .header-wrapper > img {
    width: 100%;
  }

  .heading-1 {
    color: var(--color-1);
  }
</style>
