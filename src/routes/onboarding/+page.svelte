<script>
    import { onMount, tick } from "svelte";
    import { goto } from "$app/navigation";
    
    let currentStep = 1;
    let displayedText = ""; // Text to display with typing effect
    /**
   * @type {number | undefined}
   */
    let typingInterval;
    let buttonVisible = false; // Controls the visibility of the button
    let buttonStyle = "opacity: 0; transform: scale(0.9);"; // Initial style for the button

    const steps = [
        "Welcome to Overgrowth! This application allows you to modify application icons in bulk and integrate cool plugins.",
        "Overgrowth is designed to be user-friendly and efficient. You can easily change icons for multiple applications at once.",
        "Overgrowth also supports plugins that allow you to extend its functionality. You can create your own plugins using embedded scripting."
    ];

    const typeText = (/** @type {string | any[]} */ text) => {
        displayedText = ""; // Clear the text
        let index = 0;
        clearInterval(typingInterval); // Clear any previous interval
        typingInterval = setInterval(() => {
            if (index < text.length) {
                displayedText += text[index];
                index++;
            } else {
                clearInterval(typingInterval); // Stop typing when done
            }
        }, 40); // Typing speed (40ms per character)
    };

    const goToNextStep = async () => {
        if (currentStep < steps.length) {
            currentStep++;
            typeText(steps[currentStep - 1]);
        }
        if (currentStep === 3) {
            await tick(); // Wait for DOM updates
            fadeInButton(); // Trigger the fade-in effect
        }
    };

    const goToPreviousStep = () => {
        if (currentStep > 1) {
            currentStep--;
            typeText(steps[currentStep - 1]);
            buttonVisible = false; // Hide the button if not on step 3
            buttonStyle = "opacity: 0; transform: scale(0.9);";
        }
    };

    const fadeInButton = () => {
        buttonVisible = true;
        setTimeout(() => {
            buttonStyle = "opacity: 1; transform: scale(1); transition: opacity 0.5s ease, transform 0.5s ease;";
        }, 50); // Small delay to ensure smooth transition
    };

    const openApplication = () => {
        goto("/home"); 
    };

    onMount(() => {
        typeText(steps[currentStep - 1]); // Start typing the first step on mount
    });
</script>

<h1>Overgrowth</h1>
<p>{displayedText}</p>

<div class="navigation-controls">
    <button class="arrow left-arrow" on:click={goToPreviousStep} disabled={currentStep === 1}>←</button>
    <div class="dots">
        <span class="dot {currentStep === 1 ? 'active' : ''}"></span>
        <span class="dot {currentStep === 2 ? 'active' : ''}"></span>
        <span class="dot {currentStep === 3 ? 'active' : ''}"></span>
    </div>
    <button class="arrow right-arrow" on:click={goToNextStep} disabled={currentStep === steps.length}>→</button>
</div>

{#if buttonVisible}
    <button
        class="open-app-btn"
        on:click={openApplication}
        style={buttonStyle}
    >
        Open Application
    </button>
{/if}

<style>
    @import "../../styles/onboarding.css";
    @import "../../styles/global.css";

   
</style>