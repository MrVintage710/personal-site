<script>
    import { onMount } from 'svelte';

    export let color = 1;
    export let image_url = "https://placeimg.com/640/480/tech";
    export let width = "100%";
    export let height = "100%";
    export let style = "";
    export let title = "";
    export let link = "";
    export let author = "";
    export let show_title = true;
    export let desc = "";

    function handleClick() {
        window.open(link, "_self")
    }
</script>

<div class="container" style=" --card-width : {width}; --card-height : {height}; --cursor: {link != "" ? "pointer" : "default"}; {style}" on:click={handleClick} on:keydown>
    <div class="card" style="--color: {"var(--color-" + color + ")" };">
        <div class="front">
            <div class="image" style="--image-url : {"url(" + image_url +")"};">
                <div class="title" style="display: {show_title ? 'block' : 'none'}">
                    <h1>{title}</h1>
                </div>
            </div>
        </div>
        <div class="back">
            <h2 class="back-title">{title}</h2>
            <p class="author">By {author}</p>
            <hr>
            <p class="desc">{desc}</p>
        </div>
    </div>
</div>

<style>

    @media (min-width: 2440px) {
        :root {
            --title-font-size: 1.1em;
            --title-font-line-height: 1.7em;
        }
    }

    @media (min-width: 1008px) and (max-width: 2439px) {
        :root {
            --title-font-size: 1.1em;
            --title-font-line-height: 1.7em;
        }
    }

    @media (max-width: 1007px) and (min-width: 641px) {
        :root {
            --title-font-size: 0.8em;
            --title-font-line-height: 1.7em;
        }
    }

    @media (max-width: 640px) {
        :root {
            --title-font-size: 0.8em;
            --title-font-line-height: 1.7em;
        }
    }

    .container {
        width: var(--card-width);
        height: var(--card-height);
        overflow: hidden;
    }

    .front {
        position: absolute;
        transform: translateZ(20px);
        width: 100%;
        height: 100%;
    }

    .back {
        position: absolute;
        transform: translateZ(-20px) rotateY(180deg);
        width: 100%;
        height: 100%;
        overflow-y: clip;
    }

    .back h2, .back p {
        display: block;
        box-sizing: border-box;
        margin: 0px;
        padding: 5px;
        width: 100%;
        word-wrap: normal;
        text-overflow: ellipsis;
    }

    .back p {
        font-size: 0.9em;
    }

    .desc {
        overflow: hidden;
        text-overflow: ellipsis;
        -webkit-line-clamp: 5;
        line-clamp: 5;
        -webkit-box-orient: vertical;
    }

    .card {
        position: relative;
        background-color: var(--color);
        width: 100%;
        height: 100%;
        border-radius: 15px;
        transform-style: preserve-3d;
        transition: 0.5s;
    }

    .container:hover>.card {
        transform: rotateY(180deg);
        cursor: var(--cursor);
    }

    .image {
        position: absolute;
        width: calc(100% - 20px);
        height: calc(100% - 20px);
        border-radius: 15px;
        inset: 10px;
        background-image: var(--image-url);
        perspective: 1000px;
        background-position: 50%;
        background-size: cover;
        background-repeat: no-repeat;
    }


    .title {
        position: absolute;
        backdrop-filter: blur(8px);
        text-align: center;
        width: 100%;
        border-radius: 15px;
        letter-spacing: 5px;
        line-height: 1.0;
        font-size: var(--title-font-size);
    }
</style>