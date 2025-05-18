<script lang="ts">
    import db from '$lib/db';

    export async function load() {
    try {
        const scripts = db.prepare('SELECT * FROM scripts').all();
        console.log('Fetched scripts:', scripts); // Debugging
        return { scripts };
    } catch (error) {
        console.error('Error fetching scripts:', error);
        return { scripts: [] }; // Return an empty array on error
    }
}

    export let scripts: any[] = []; // This will be populated by the `load` function

    function installScript(scriptId: number) {
        console.log(`Installing script with ID: ${scriptId}`);
        // Add logic to handle script installation
    }

    function uninstallScript(scriptId: number) {
        console.log(`Uninstalling script with ID: ${scriptId}`);
        // Add logic to handle script uninstallation
    }
</script>

<style>
    @import "../../styles/home.css";
    @import "../../styles/global.css";

    .marketplace {
        display: flex;
        flex-wrap: wrap;
        gap: 1rem;
    }

    .script-card {
        border: 1px solid #ccc;
        border-radius: 8px;
        padding: 1rem;
        width: 300px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }

    .script-card img {
        max-width: 100%;
        height: auto;
        border-radius: 4px;
    }

    .script-card h2 {
        font-size: 1.25rem;
        margin: 0.5rem 0;
    }

    .script-card p {
        font-size: 0.9rem;
        color: #555;
    }

    .script-card button {
        margin-top: 0.5rem;
        padding: 0.5rem 1rem;
        border: none;
        border-radius: 4px;
        cursor: pointer;
    }

    .install-btn {
        background-color: #4caf50;
        color: white;
    }

    .uninstall-btn {
        background-color: #f44336;
        color: white;
    }
</style>

<div class="container">
    <h1>Marketplace</h1>
    <div class="marketplace">
        {#each scripts as script}
            <div class="script-card">
                <img src={`/icons/${script.icon}`} alt={script.name} />
                <h2>{script.name}</h2>
                <p>{script.description}</p>
                <small>By {script.author} on {script.last_updated}</small>
                <div>
                    <button class="install-btn" on:click={() => installScript(script.id)}>Install</button>
                    <button class="uninstall-btn" on:click={() => uninstallScript(script.id)}>Uninstall</button>
                </div>
            </div>
        {/each}
    </div>
</div>