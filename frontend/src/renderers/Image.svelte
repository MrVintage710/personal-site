<script>
  import {current_path} from "../lib/current_path"
  import {get_body, get_cards} from "../assets/mtg"
  import {onMount, beforeUpdate} from "svelte"
  import DisplayCard from "../lib/DisplayCard.svelte";
  import { each } from "svelte/internal";

  export let href = ''
  export let title = undefined
  export let text = ''

  let path;
  let card_names = [];
  let cards = [];

  current_path.subscribe((current_path) => {
    path = current_path
  })
  
  onMount(async () => {
    if(text === "mtg") {
      card_names = get_cards(title);
      let res = await fetch("/card_query", {
        method: "POST",
        headers: {
          'Content-Type': 'application/json'
        },
        body : JSON.stringify(get_body(card_names))
      });
      cards = await res.json();
      card_names = card_names;
      console.log(cards);
    }
  })

  $: card_names = card_names
  $: cards = cards
</script>

{#if text === "video"}
  <div class="video-container">
    <iframe width="100%" height="100%" src="{href.replace("watch?v=", "embed/")}" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>
  </div>
  {:else if text === "card"}
  <div class="card-display">
    <DisplayCard front_image={"/assets/" + path + "/" + href}></DisplayCard>
  </div>
  {:else if text === "mtg"}
  <div class="card-flex">
    {#each card_names as name, i}
      <div class="card-display">
        <DisplayCard 
        front_image={cards[name] ? cards[name].front : ""}
        back_image={cards[name] ? cards[name].back ? cards[name].back : "" : ""}
        ></DisplayCard>
      </div>
    {/each}
  </div>
  {:else}
  <img src={"assets/" + path + "/" + href} {title} alt={text}>
{/if}

<style>
  img {
    width: 100%;
  }

  .video-container {
    position: relative;
    padding-bottom: 56.25%;
  }

  .video-container > iframe {
    /* display: block;
    margin: 0 auto; */
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
  }

  .card-display {
    width: 250px;
    height: 350px;
  }

  .card-flex {
    display: flex;
    align-items: center;
    justify-content: space-around;
    align-content: space-between;
    flex-direction: row;
    width: 100%;
    flex-wrap: wrap;
    gap: 30px;
  }
</style>