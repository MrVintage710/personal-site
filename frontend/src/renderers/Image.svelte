<script>
  export let href = ''
  export let title = undefined
  export let text = ''
</script>

<img src={"assets/" + href} {title} alt={text}>

<style>
  img {
    width: 100%;
  }
</style>