<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { app } from "$lib/index";

    onMount(() => {
        const init = async () => {
            await app.init();
            setTimeout(() => {
                goto("/onboarding");
            }, 5000); // Delay of 5 seconds
        };
        init();
    });
</script>

<div class="loading-screen">
    <h1>Overgrowth</h1>
    <div class="loading-slider">
        <div class="slider"></div>
    </div>
</div>

<style>
    @import "../styles/onboarding.css";

    .loading-screen {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        height: 100vh;
        background-color: #1a1a1a;
        color: #fff;
        font-family: Arial, sans-serif;
        text-align: center;
    }

    .loading-screen h1 {
        font-size: 3rem;
        margin-bottom: 2rem;
    }

    .loading-slider {
        width: 80%;
        max-width: 400px;
        height: 8px;
        background-color: #333;
        border-radius: 4px;
        overflow: hidden;
        position: relative;
    }

    .slider {
        width: 30%;
        height: 100%;
        background-color: #0078d4;
        animation: slide 5s infinite; /* Matches the delay duration */
    }

    @keyframes slide {
        0% {
            transform: translateX(-100%);
        }
        50% {
            transform: translateX(50%);
        }
        100% {
            transform: translateX(100%);
        }
    }
</style>