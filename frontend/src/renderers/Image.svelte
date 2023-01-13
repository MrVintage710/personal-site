<script>
  import {current_path} from "../lib/current_path"

  export let href = ''
  export let title = undefined
  export let text = ''

  let path;

  current_path.subscribe((current_path) => {
    path = current_path
  })
</script>

<img src={"assets/" + path + "/" + href} {title} alt={text}>

<style>
  img {
    width: 100%;
  }
</style>