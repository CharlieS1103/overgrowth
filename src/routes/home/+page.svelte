<script>
    let menuItems = [
        { name: "Marketplace", link: "/marketplace" },
        { name: "Settings", link: "/settings" },
        { name: "Profile", link: "/profile" },
        { name: "Help", link: "/help" }
    ];
</script>

<style>
        @import "../../styles/home.css";
        @import "../../styles/global.css";
</style>

<div class="container">
    <div class="sidebar">
        <h2>Menu</h2>
        {#each menuItems as item}
            <div class="menu-item">
                <a href={item.link}>{item.name}</a>
            </div>
        {/each}
    </div>
    <div class="content">
        <h1>Welcome to Overgrowth</h1>
        <div class="feature">
            <h3>Feature 1</h3>
            <p>Explore the marketplace to find amazing content.</p>
        </div>
        <div class="feature">
            <h3>Feature 2</h3>
            <p>Customize your settings to enhance your experience.</p>
        </div>
        <div class="feature">
            <h3>Feature 3</h3>
            <p>Manage your profile and keep your information up to date.</p>
        </div>
    </div>
</div>