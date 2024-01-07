<template>
  <nav ref="navbar">
    <NuxtLink class="brand" to="/">hartex</NuxtLink>
    <div class="links">
      <NuxtLink to="/" target="_blank">
        <div class="i-carbon-notebook-reference"></div>
      </NuxtLink>
      <NuxtLink to="https://discord.com/invite/Xu8453VBAv" target="_blank">
        <div class="i-carbon-logo-discord"></div>
      </NuxtLink>
      <NuxtLink to="https://github.com/TeamHarTex/HarTex" target="_blank">
        <div class="i-carbon-logo-github"></div>
      </NuxtLink>
    </div>
  </nav>
</template>

<style scoped lang="postcss">
nav {
  @apply fixed flex justify-between items-center;
  @apply w-full py-18 transition-opacity;

  .brand {
    @apply text-4xl font-600;
  }

  .links {
    @apply flex;

    a {
      @apply text-3xl mx-3;
    }

    a:first-child {
      @apply ml-0;
    }

    a:last-child {
      @apply mr-0;
    }
  }
}
</style>

<script setup lang="ts">
const navbar: Ref<HTMLElement | null> = ref(null);

let showNavbar = true;
let lastScrollPosition = 0;

onMounted(() => {
  window.addEventListener('scroll', onScroll);
});

function onScroll() {
  const currentScrollPosition = window.scrollY;

  if (currentScrollPosition < 0) {
    return;
  }

  if (Math.abs(currentScrollPosition - lastScrollPosition) < 30) {
    return
  }

  showNavbar = currentScrollPosition < lastScrollPosition;

  lastScrollPosition = currentScrollPosition;

  if (navbar.value) {
    if (!showNavbar) {
      navbar.value.style.opacity = "0";
      navbar.value.style.pointerEvents = 'none';
    } else {
      navbar.value.style.opacity = "100";
      navbar.value.style.pointerEvents = 'auto';
    }
  }
}
</script>