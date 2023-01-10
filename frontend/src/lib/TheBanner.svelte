<script>
    import { createEventDispatcher, onMount } from 'svelte';
    import { draw } from 'svelte/transition'

    export let current_page;
    let socket_current_class = "decoration-light";
    let socket_text_current_class = "color-light"

    const dispatch = createEventDispatcher();

    function onTabClick(value) {
        dispatch('tab-clicked', {
            value : value === current_page ? -1 : value 
        })
    }

    function getSocketClass(value) {
        switch(value) {
            case 1:
                return "decoration-1";
            case 2:
                return "decoration-2"
            case 3:
                return "decoration-3"
            case 4:
                return "decoration-4"
            default:
                return "decoration-light"
        }
    }

    function getSocketText(value) {
        switch(value) {
            case 1:
                return "color-2"
            case 2:
                return "color-3"
            case 3:
                return "color-4"
            case 4:
                return "color-5"
            default:
                return "color-light"
        }
    }

    $: socket_current_class = getSocketClass(current_page)
    $: socket_text_current_class = getSocketText(current_page)
</script>

<div class="main-section-container">
    <div class="title-container">
      <h1>Brooks Palin</h1>
    </div>
    <div class="main-section panel-1" class:selected="{current_page === 1}" on:click={() => {onTabClick(1)}} on:keydown>
        <p class="courier">Bio</p>
        <svg class="tech-lines panel-lines" width="100%" height="100%">
            <g>
              <line class="decoration-1" x1="0" y1="0" x2="0px" y2="100vh"/>
              <circle class="decoration-1" cx="0" cy="calc(100vh + 15px)" r="15"/>
            </g>
        </svg>
    </div>
    <div class="main-section panel-2" class:selected = "{current_page === 2}" on:click={() => {onTabClick(2)}} on:keydown>
      <p class="courier">Projects</p>  
      <svg class="tech-lines panel-lines" width="100%" height="100%">
        <g>
          <line class="decoration-2" x1="0" y1="0" x2="0" y2="100vh"/>
          <circle class="decoration-2" cx="0" cy="calc(100vh + 15px)" r="15"/>
        </g>
      </svg>
    </div>
    <div class="main-section panel-3" class:selected = "{current_page === 3}" on:click={() => {onTabClick(3)}} on:keydown>
      <p class="courier">Blog</p>  
      <svg class="tech-lines panel-lines" width="100%" height="100%">
        <g>
          <line class="decoration-3" x1="0" y1="0" x2="0" y2="100vh"/>
          <circle class="decoration-3" cx="0" cy="calc(100vh + 15px)" r="15"/>
        </g>
      </svg>
    </div>
    <div class="main-section panel-4" class:selected = "{current_page === 4}" on:click={() => {onTabClick(4)}} on:keydown>
      <p class="courier">Contact</p> 
      <svg class="tech-lines panel-lines" width="100%" height="100%" >
        <g>
          <line class="decoration-4" x1="0" y1="0" x2="0" y2="100vh"/>
          <circle class="decoration-4" cx="0" cy="calc(100vh + 15px)" r="15"/>
        </g>
      </svg>
    </div>
    <div class="main-section panel-5"></div>
    <div class="display-text-container">
        <p class="{socket_text_current_class + " courier"}">Display Out</p>
    </div>
    <svg width="calc(100%)" height="100%" class="tech-lines connector-line">
        <g>
          {#if current_page === 1}
            <line transition:draw="{{duration: 500}}" class="connector-1" x2="calc(10vw + 10px)" y2="15" x1="calc(90vw - 15px)" y1="15"></line>
          {/if}
          {#if current_page === 2}
            <line transition:draw="{{duration: 500}}" class="connector-2" x2="calc(30vw + 10px)" y2="15" x1="calc(90vw - 15px)" y1="15"></line>
          {/if}
          {#if current_page === 3}
            <line transition:draw="{{duration: 500}}" class="connector-3" x2="calc(50vw + 10px)" y2="15" x1="calc(90vw - 15px)" y1="15"></line>
          {/if}
          {#if current_page === 4}
            <line transition:draw="{{duration: 500}}" class="connector-4" x2="calc(70vw + 10px)" y2="15" x1="calc(90vw - 15px)" y1="15"></line>
          {/if}
          {#if current_page > 4 || current_page < 1}
            <line transition:draw="{{duration: 500}}" class="connector-light" x2="calc(110vw)" y2="15" x1="calc(90vw + 15px)" y1="15"></line>
          {/if}
          <circle 
          class="{socket_current_class}" 
          style="transition: 0.5s; delay: 0.5"
          cx="calc(90vw)" 
          cy="15" 
          r="15"/>
        </g>
    </svg>
    <div class="profile-pic-container">
        <div></div>
    </div>
</div>

<style>

  /* Variables */
  @media (min-width: 2440px) {
    :root {
      --header-bottom: 500px;
      --profile-pic-size: 250px;
      --profile-pic-size: 200px;
      --profile-pic-display-status: block;

      /* Title */
      --title-x: calc(20vw - 50%);
      --title-y: 50px;

      /* Display Port */
      --display-port-offset: 0px;

      /* Panels */
      --panel-1-horizontal-pos: 10vw;
      --panel-2-horizontal-pos: 30vw;
      --panel-3-horizontal-pos: 50vw;
      --panel-4-horizontal-pos: 70vw;
      --panel-5-horizontal-pos: 90vw;

      --panel-5-display-status: block;

      --panel-hover-distance: 10px;
      --panel-extention-distance: 60px;
    }
  }

  @media (min-width: 1008px) and (max-width: 2439px) {
    :root {
      --header-bottom: 400px;
      --profile-pic-size: 250px;
      --profile-pic-size: 200px;
      --profile-pic-display-status: block;

      /* Title */
      --title-x: calc(20vw - 50%);
      --title-y: 50px;

      /* Display Port */
      --display-port-offset: 0px;

      /* Panels */
      --panel-1-horizontal-pos: 10vw;
      --panel-2-horizontal-pos: 30vw;
      --panel-3-horizontal-pos: 50vw;
      --panel-4-horizontal-pos: 70vw;
      --panel-5-horizontal-pos: 90vw;

      --panel-5-display-status: block;

      --panel-hover-distance: 10px;
      --panel-extention-distance: 60px;
    }
  }

  @media (max-width: 1007px) and (min-width: 641px) {
    :root {
      --header-bottom: 325px;
      --profile-pic-size: 200px;
      --profile-pic-display-status: none;
      --panel-5-display-status: block;

      /* Title */
      --title-x: calc(25vw - 50%);
      --title-y: 25px;

      /* Display Port */
      --display-port-offset: 0px;

      /* Panels */
      --panel-1-horizontal-pos: 10vw;
      --panel-2-horizontal-pos: 30vw;
      --panel-3-horizontal-pos: 50vw;
      --panel-4-horizontal-pos: 70vw;
      --panel-5-horizontal-pos: 90vw;

      --panel-hover-distance: 0px;
      --panel-extention-distance: 60px;
    }
  }

  @media (max-width: 640px) {
    :root {
      --header-bottom: 325px;
      --profile-pic-size: 200px;
      --profile-pic-display-status: none;
      --panel-5-display-status: none;

      /* Title */
      --title-x: calc(50vw - 50%);
      --title-y: 25px;

      /* Display Port */
      --display-port-offset: -100vh;
      --display-port-text: none;

      /* Panels */
      --panel-1-horizontal-pos: 15vw;
      --panel-2-horizontal-pos: 40vw;
      --panel-3-horizontal-pos: 65vw;
      --panel-4-horizontal-pos: 90vw;
      --panel-5-horizontal-pos: 100vw;

      --panel-hover-distance: 0px;
      --panel-extention-distance: 30px;
    }
  }

  .connector-line {
    position: absolute;
    pointer-events: none;
  }

  .connector-line>g {
    position: absolute;
    transform: translate(0px, calc(var(--header-bottom) + var(--display-port-offset)));
    width: 100%;
  }

  .connector-1 {
    stroke: var(--color-2);
    stroke-width: 8px;
    fill: none;
  }

  .connector-2 {
    stroke: var(--color-3);
    stroke-width: 8px;
    fill: none;
  }

  .connector-3 {
    stroke: var(--color-4);
    stroke-width: 8px;
    fill: none;
  }

  .connector-4 {
    stroke: var(--color-5);
    stroke-width: 8px;
    fill: none;
  }

  .connector-light {
    stroke: var(--color-light);
    stroke-width: 8px;
    fill: none;
  }

  /* Title */
  .title-container {
    position: absolute;
    color: var(--color-light);
    z-index: 2;
    padding: 5px;
    /* background-color: var(--color-dark-subtle); */
    backdrop-filter: blur(8px);
    border-radius: 10px;
    transform: translate(var(--title-x), var(--title-y));
  }

  .title-container h1 {
    font-size: 40pt;
    position: relative;
    z-index: 2;
  }

  /* Profile Picture */
  .profile-pic-container {
    width: 250px;
    height: 250px;
    border-radius: 50%;
    background-color: var(--color-dark);
    position: absolute;
    z-index: 2;
    padding: 15px;
    box-sizing: border-box;
    transform: translate(min(calc(90vw - 50%), calc(100vw - 275px)), 10px);
    display: var(--profile-pic-display-status);
  }

  .profile-pic-container div {
    width: 100%;
    height: 100%;
    transform-origin: center;
    background-color: white;
    position: relative;
    border-radius: 50%;
  }

  /* Panel Master Class */
  .main-section {
    width: 100vw;
    height: 100vh;
    border-radius: 15px;
    transition: 0.5s;
    overflow: hidden;
  }

  .main-section-container {
    width: 100vw;
    height: calc(var(--header-bottom) + var(--panel-extention-distance));
  }
  
  .main-section p {
    position: absolute;
    transform: translate(calc(50px - 100%), 92vh) rotateZ(90deg);
    transform-origin: right;
    text-align: right;
    font-weight: 500;
    font-size: large;
    color: var(--color-light);
    margin: 0px;
  }

  /* Panel Lines */
  .panel-lines>g {
    transform: translate(30px, -50px);
  }

  /* Panel 1 */
  .main-section.panel-1 {
    transform-origin: bottom left;
    transform: translate(var(--panel-1-horizontal-pos), calc(var(--header-bottom) - 100vh)) rotateZ(-45deg);
    background-color: var(--color-1);
    position: absolute;
  }

  .main-section.panel-1.selected {
    transform: translate(var(--panel-1-horizontal-pos), calc((var(--header-bottom) - 100vh) + var(--panel-extention-distance))) rotateZ(-45deg);
  }

  .main-section.panel-1:hover:not(.selected){
    transform: translate(var(--panel-1-horizontal-pos), calc((var(--header-bottom) - 100vh) + var(--panel-hover-distance))) rotateZ(-45deg);
  }

  /* Panel 2 */
  .main-section.panel-2 {
    transform-origin: bottom left;
    transform: translate(var(--panel-2-horizontal-pos), calc(var(--header-bottom) - 100vh)) rotateZ(-45deg);
    background-color: var(--color-2);
    position: absolute;
  }

  .main-section.panel-2.selected {
    transform: translate(var(--panel-2-horizontal-pos), calc((var(--header-bottom) - 100vh) + var(--panel-extention-distance))) rotateZ(-45deg);
  }

  .main-section.panel-2:hover:not(.selected){
    transform: translate(var(--panel-2-horizontal-pos), calc((var(--header-bottom) - 100vh) + var(--panel-hover-distance))) rotateZ(-45deg);
  }

  /* Panel 3 */
  .main-section.panel-3 {
    transform-origin: bottom left;
    transform: translate(var(--panel-3-horizontal-pos), calc(var(--header-bottom) - 100vh)) rotateZ(-45deg);
    background-color: var(--color-3);
    position: absolute;
  }

  .main-section.panel-3.selected {
    transform: translate(var(--panel-3-horizontal-pos), calc((var(--header-bottom) - 100vh) + var(--panel-extention-distance))) rotateZ(-45deg);
  }

  .main-section.panel-3:hover:not(.selected){
    transform: translate(var(--panel-3-horizontal-pos), calc((var(--header-bottom) - 100vh) + var(--panel-hover-distance))) rotateZ(-45deg);
  }

  /* Panel 4 */
  .main-section.panel-4 {
    transform-origin: bottom left;
    transform: translate(var(--panel-4-horizontal-pos), calc(var(--header-bottom) - 100vh)) rotateZ(-45deg);
    background-color: var(--color-4);
    position: absolute;
  }

  .main-section.panel-4.selected {
    transform: translate(var(--panel-4-horizontal-pos), calc((var(--header-bottom) - 100vh) + var(--panel-extention-distance))) rotateZ(-45deg);
  }

  .main-section.panel-4:hover:not(.selected){
    transform: translate(var(--panel-4-horizontal-pos), calc((var(--header-bottom) - 100vh) + var(--panel-hover-distance))) rotateZ(-45deg);
  }

  /* Panel 5 */
  .main-section.panel-5 {
    transform-origin: bottom left;
    transform: translate(var(--panel-5-horizontal-pos), calc(var(--header-bottom) - 100vh)) rotateZ(-45deg);
    background-color: var(--color-5);
    position: absolute;
    border-radius: 200px;
    display: var(--panel-5-display-status);
  }
  
  /* Styling for the 'Display Out' text. */
  .display-text-container {
    position: absolute;
    transform-origin: center;
    transform: translate(calc(90vw - 50%), calc(var(--header-bottom) - 55px));
    text-align: center;
    max-width: 70px;
    min-width: 65px;
    width: 10vw;
    display: var(--display-port-text);
  }

  .display-text-container p {
    margin: 0;
    transition: 0.5s;
    background-color: var(--color-dark);
    display: var(--display-port-text);
  }
</style>