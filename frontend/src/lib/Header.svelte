<script>
  export let height = "50px";
  export let history = [];
</script>

<div class="header-container" style={"--height: " + height}>
    <div class="crumb" role="link" style="--color: var(--color-3);">
      <div class="crumb-body">
        <p>Test</p>
        <div class="crumb-tip"></div>
      </div>
    </div>
    <div class="crumb" role="link" style="--color: var(--color-2);">
      <div class="crumb-body">
        <a href="#">Test</a>
        <div class="crumb-tip"></div>
      </div>
    </div>
    <div class="crumb" role="link" style="--color: var(--color-1);">
      <div class="crumb-body">
        <a href="#">This is a longer test</a>
        <div class="crumb-tip"></div>
      </div>
    </div>
</div>

<style>
  .header-container {
    height: var(--height);
    position: sticky;
    position: -webkit-sticky;
    display: flex;
    flex-direction: row-reverse;
    justify-content: flex-end;
    top: 0;
    overflow-x: scroll;
    overflow-y: hidden;
    -ms-overflow-style: none;
    scrollbar-width: none;
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
    padding-left: calc(var(--tip-size)/2 + 5px);
    padding-right: calc(var(--tip-size));
    display: flex;
    justify-content: center;
    align-content: center;
    flex-direction: column;
  }

  .crumb-body > p {
    margin: 0px;
  }
</style>