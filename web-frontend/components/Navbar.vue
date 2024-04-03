<!--
SPDX-License-Identifier: AGPL-3.0-only

This file is part of HarTex.

HarTex
Copyright (c) 2021-2024 HarTex Project Developers

HarTex is free software; you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation; either version 3 of the License, or
(at your option) any later version.

HarTex is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along
with HarTex. If not, see <https://www.gnu.org/licenses/>.
-->

<template>
  <nav ref="navbar">
    <NuxtLink class="brand" to="/">hartex</NuxtLink>
    <div class="links">
      <NuxtLink to="/" target="_blank">
        <div class="i-carbon:notebook-reference"></div>
      </NuxtLink>
      <NuxtLink to="https://discord.com/invite/Xu8453VBAv" target="_blank">
        <div class="i-carbon:logo-discord"></div>
      </NuxtLink>
      <NuxtLink to="https://github.com/TeamHarTex/HarTex" target="_blank">
        <div class="i-carbon:logo-github"></div>
      </NuxtLink>
      <NuxtLink to="https://github.com/TeamHarTex/HarTex/blob/nightly/CHANGELOG.md" target="_blank">
        <div class="i-carbon:catalog"></div>
      </NuxtLink>
    </div>
  </nav>
</template>

<style scoped lang="postcss">
nav {
  @apply fixed flex justify-between items-center z-99;
  @apply w-full py-12 md:py-18 transition-opacity;

  .brand {
    @apply text-3xl md:text-4xl font-600;
  }

  .links {
    @apply flex;

    a {
      @apply mt-1 text-3xl mx-3;
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
