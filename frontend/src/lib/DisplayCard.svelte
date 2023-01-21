<script>
    import { onMount } from 'svelte';

    import VanillaTilt from 'vanilla-tilt'
    export let front_image = "";
    export let back_image = "";

    let card_element;
    let card_front;
    let card_back;
    let flipped = false;
    let tilt = {};

    onMount(() => {
        tilt = new VanillaTilt(card_front);
    })

    function flip() {
        if(flipped) {
            //Destroy Card Back
            tilt = new VanillaTilt(card_front);
            flipped = false;
        } else {
            tilt.destroy();
            flipped = true;
        }
    }
</script>

{#if back_image !== ""}
<div role="button" class="flip-button" on:click={flip} on:keyup><i class="fa-solid fa-rotate"></i></div>
{/if}

<div class="card" bind:this={card_element} data-tilt data-tilt-scale="1.1" style="{"--rotation : " + (flipped ? "180deg" : "0deg")}">
    <div bind:this={card_front} class="image image-front" style="--image-url : {"url(" + front_image +")"};"></div>
    <div bind:this={card_back} class="image image-back" style="--image-url : {"url(" + back_image +")"};"></div>
</div>

<style>

    .card {
        position: relative;
        width: 100%;
        height: 100%;
        border-radius: 15px;
        transform-style: preserve-3d;
        transform: perspective(1000px) rotateY(var(--rotation));
        transition: 0.5s;
    }

    .image {
        position: absolute;
        width: 100%;
        height: 100%;
        border-radius: 15px;
        background-image: var(--image-url);
        background-position: 50%;
        background-size: cover;
        background-repeat: no-repeat;
        
    }

    .image-front {
        transform: translateZ(5px);
    }

    .image-back {
        transform: translateZ(-5px) rotateY(180deg);
    }

    .flip-button {
        position: absolute;
        width: 50px;
        height: 50px;
        background-color: var(--color-accent);
        border-radius: 50%;
        z-index: 50;
        transform: translate(calc(250px - 100% - 10px), calc(350px - 100% - 10px));

        display: flex;
        justify-content: center;
        align-content: center;
        flex-direction: column;
        text-align: center;
    }
</style>