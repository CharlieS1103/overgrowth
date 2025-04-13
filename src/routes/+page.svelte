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
    @import "../styles/global.css";
    @import "../styles/loading.css";
</style>