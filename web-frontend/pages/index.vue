<template>
    <div>
        <div class="title">
            <div class="main-title">
                <h1 class="title-text">
                    HarTex<span class="dev-tag">DEV</span>
                </h1>
                <h1 class="slogan-text">
                    Discord Server Management.
                    <br>
                    Reimagined.
                </h1>
                <p class="brief-description-text">
                    Empowers administrators to manage Discord Servers effectively.
                    <br>
                    Makes use of the latest cutting-edge features on Discord.
                </p>
                <div class="buttons">
                    <a class="get-started-button">
                        Get Started
                    </a>
                    <a class="learn-more-button">
                        Learn More
                    </a>
                </div>
            </div>
        </div>
        <div class="content">

        </div>
    </div>
</template>

<style scoped>
.title {
    @apply bg-gradient-to-b from-main-background to-main-background-secondary;
    @apply pb-[1.5rem]
}

.main-title {
    @apply pt-[13vh] pd-[5vh] px-[13rem];
}

.title-text {
    @apply mb-[1rem];
    @apply font-bold text-[2.75vw];
}

.dev-tag {
    @apply border-solid border-[1px] border-main-primary;
    @apply ml-[8px] position-relative px-[8px] py-[4px] top-[-25px];
    @apply rounded-2;
    @apply text-[0.75vw] text-main-primary;
}

.slogan-text {
    @apply font-black text-[3.5vw];
}

.brief-description-text {
    @apply pt-[16px];
    @apply font-[550] text-[1.25vw];
}

.buttons {
    @apply mt-[3rem];
}

.get-started-button {
    @apply bg-main-primary;
    @apply px-8 py-4 mr-2;
    @apply font-semibold text-[1.25rem];
    @apply rounded-2;
}

.learn-more-button {
    @apply bg-main-secondary;
    @apply px-8 py-4;
    @apply font-semibold text-[1.25rem];
    @apply rounded-2;
}

.content {
    @apply h-[719px];
    @apply bg-main-background-secondary
}
</style>
