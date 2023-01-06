<script>
    import { onMount } from 'svelte';

    import VanillaTilt from 'vanilla-tilt'

    export let color = 1;
    export let image_url = "https://placeimg.com/640/480/tech";
    export let title = "";

    let card_element;

    onMount(() => {
        const tilt = new VanillaTilt(card_element, );
    })
</script>

<div class="card" style="--color: {"var(--color-" + color + ")" };" bind:this={card_element} data-tilt data-tilt-scale="1.1">
    <div class="image" style="--image-url : {"url(" + image_url +")"};">
        <div class="title">
            <h1>{title}</h1>
        </div>
    </div>
</div>

<style>

    .card {
        position: relative;
        background-color: var(--color);
        width: 100%;
        height: 100%;
        border-radius: 15px;
        left: 20px;
        transform-style: preserve-3d;
        transform: perspective(1000px)
    }

    .image {
        position: absolute;
        width: calc(100% - 20px);
        height: calc(100% - 20px);
        border-radius: 15px;
        inset: 10px;
        background-image: var(--image-url);
        background-position: 50%;
        background-size: cover;
        background-repeat: no-repeat;
        transform: translateZ(20px)
    }


    .title {
        position: absolute;
        backdrop-filter: blur(8px);
        text-align: center;
        transform: translateZ(50px);
        width: 100%;
    }
</style>