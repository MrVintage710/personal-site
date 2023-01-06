<script>
    import { onMount } from 'svelte';

    export let color = 1;
    export let image_url = "https://placeimg.com/640/480/tech";
    export let width = "100%";
    export let height = "100%";
    export let style = "";
    export let title = "";
    export let link = "";
</script>

<div class="container" style=" --card-width : {width}; --card-height : {height}; --cursor: {link != "" ? "pointer" : "default"}; {style}">
    <div class="card" style="--color: {"var(--color-" + color + ")" };">
        <div class="front">
            <div class="image" style="--image-url : {"url(" + image_url +")"};">
                <div class="title" style="">
                    <h1>{title}</h1>
                </div>
            </div>
        </div>
        <div class="back">
            <p>Test</p>
        </div>
    </div>
</div>

<style>

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
        font-size: 1.0em;
    }
</style>