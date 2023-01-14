<script>
  export let height = "50px";
  export let path = [];

  function onClick(item) {
    console.log(item)
    item.scrollIntoView({block: "center", inline: "nearest", behavior : "smooth"})
  }
</script>

<div class="header-container" style={"--height: " + height}>
  <div class="flex-container">
    {#each path.reverse() as item, i}
      <div class="crumb" role="link" style="{"--color: var(--color-" + (i%5 + 1) +")"}" on:click={() => onClick(item)} on:keyup>
        <div class="crumb-body">
          <p>{item.innerText}</p>
          <div class="crumb-tip"></div>
        </div>
      </div>
    {/each}
    <div class="home" on:click={() => {window.open("/", "_self")}} on:keyup role="link"><i class="fa-solid fa-house"></i></div>
  </div>
  <div class="menu-button"><i class="fa-solid fa-bars"></i></div>
</div>

<style>
  .header-container {
    height: var(--height);
    position: sticky;
    position: -webkit-sticky;
    overflow-x: scroll;
    overflow-y: hidden;
    -ms-overflow-style: none;
    scrollbar-width: none;
    z-index: 100;
    top: 0;
  }

  .flex-container {
    display: flex;
    flex-direction: row-reverse;
    justify-content: flex-end;
  }

  .home {
    width: var(--height);
    height: var(--height);
    background-color: var(--color-accent);
    display: flex;
    justify-content: center;
    align-content: center;
    flex-direction: column;
    text-align: center;
    border-radius: 0px 50% 50% 0px;
    margin-right: calc(var(--height)/-2);
  }

  .home:hover {
    cursor: pointer;
  }

  .menu-button {
    width: var(--height);
    height: var(--height);
    background-color: var(--color-accent);
    border-radius: 50% 0px 0px 50%;
    position: absolute;
    right: 0px;
    top: 0px;
    display: flex;
    justify-content: center;
    align-content: center;
    flex-direction: column;
    text-align: center;
  }

  .menu-button:hover {
    cursor: pointer;
  }

  .header-container:nth-last-child(0) {
    padding-left: calc(var(--tip-size)/2 + 15px);
    background-color: red;
  }

  .header-container::-webkit-scrollbar {
    display: none;
  }

  .crumb {
    --crumb-height: var(--height);
    --tip-size: calc(var(--crumb-height) / 1.41421356237);
    --z: 2;
  }

  .crumb:hover {
    cursor: pointer;
  }

  .crumb-tip {
    background-color: var(--color);
    width: var(--tip-size);
    height: var(--tip-size);
    border-radius: 0px 15px 0px 0px;
    position: absolute;
    transform: rotateZ(45deg) translateZ(-10px);
    transform-origin: center;
    right: calc(var(--tip-size)/-2);
    bottom: calc(var(--crumb-height)/6);
    z-index: 100;
  }

  .crumb-body {
    background-color: var(--color);
    height: var(--crumb-height);
    position: relative;
    z-index: -100;
    padding-left: calc(var(--tip-size)/2 + 15px);
    /* padding-right: calc(var(--tip-size)); */
    display: flex;
    justify-content: center;
    align-content: center;
    flex-direction: column;
  }

  .crumb-body > p {
    margin: 0px;
    z-index: 150;
  }
</style>