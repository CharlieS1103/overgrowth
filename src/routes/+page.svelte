<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/tauri";
    import { app } from "$lib/index"
    const runMacLogic = async () => {
        try {
            await invoke("mac_logic");
            console.log("Mac logic executed successfully!");
        } catch (error) {
            console.error("Error running mac logic:", error);
        }
    };

    onMount(() => {
        const init = async () => {
            await app.init();
            setTimeout(() => {
                goto("/onboarding");
            }, 2000); // Delay of 5 seconds
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