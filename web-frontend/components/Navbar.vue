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
    <NuxtLink class="brand" to="/">HarTex</NuxtLink>
    <div class="links">
      <NuxtLink to="https://discord.com/invite/Xu8453VBAv" target="_blank">
        <div class="i-carbon:logo-discord"></div>
      </NuxtLink>
      <NuxtLink to="https://github.com/TeamHarTex/HarTex" target="_blank">
        <div class="i-carbon:logo-github"></div>
      </NuxtLink>
      <NuxtLink @click="toggleMenu()">
        <div class="i-carbon:catalog"></div>
      </NuxtLink>
    </div>
  </nav>
  <div ref="sidebar" :class="{ 'hidden!': !isOpened }" class="sidebar">
    <div class="navigation">
      <h2 class="subheading">Changelogs:</h2>
      <button class="close" @click="toggleMenu()">
        <div class="i-carbon-right-panel-close-filled"></div>
      </button>
    </div>
    <div class="whitespace"></div>
    <div class="whitespace"></div>
    <div class="logs">
      <button>Example Log</button>
    </div>
  </div>
  <div :class="{ hidden: !isOpened }" class="overlay"></div>
</template>

<style scoped lang="postcss">
nav {
  @apply fixed flex justify-between items-center z-97;
  @apply w-full py-12 md:py-18 transition-opacity;

  .brand {
    @apply text-3xl md:text-4xl font-bold;
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

.sidebar {
  @apply fixed right-0 w-[35vw] h-screen z-99;
  @apply flex flex-col p-15;
  @apply bg-secondary text-primary;

  border-top-left-radius: 50px;
  border-bottom-left-radius: 50px;

  .navigation {
    @apply flex justify-between items-center w-full h-fit;

    .close {
      @apply text-4xl text-primary;
      @apply transition-color duration-300;

      &:hover {
        @apply text-tertiary;
      }
    }

    h2 {
      @apply inline select-none;
    }
  }

  .logs {
    button {
      @apply text-4xl text-primary font-600 uppercase;
      @apply border-solid border-b-2 w-full text-left pb-5;

      &:hover {
        @apply text-tertiary border-b-tertiary;
      }
    }
  }
}

.overlay {
  @apply fixed w-screen h-screen z-98;

  background: rgba(0, 0, 0, 0.5);
}
</style>

<script setup lang="ts">
const { $gsap } = useNuxtApp();

const navbar: Ref<HTMLElement | null> = ref(null);

const isOpened = ref(false);

let showNavbar = true;
let lastScrollPosition = 0;

function onScroll() {
  const currentScrollPosition = window.scrollY;

  if (currentScrollPosition < 0) {
    return;
  }

  if (Math.abs(currentScrollPosition - lastScrollPosition) < 30) {
    return;
  }

  showNavbar = currentScrollPosition < lastScrollPosition;

  lastScrollPosition = currentScrollPosition;

  if (navbar.value) {
    if (!showNavbar) {
      navbar.value.style.opacity = "0";
      navbar.value.style.pointerEvents = "none";
    } else {
      navbar.value.style.opacity = "100";
      navbar.value.style.pointerEvents = "auto";
    }
  }
}

function toggleMenu() {
  if (!isOpened.value) {
    isOpened.value = true;

    $gsap.fromTo(
      ".sidebar",
      {
        x: "45vw",
      },
      {
        x: 0,
        delay: 0.5,
        duration: 1.5,
        ease: "expo.out",
      }
    );

    $gsap.fromTo(
      ".overlay",
      {
        opacity: 0,
      },
      {
        opacity: 1,
        duration: 1.5,
        ease: "expo.out",
      }
    );
  } else {
    $gsap.fromTo(
      ".sidebar",
      {
        x: 0,
      },
      {
        x: "45vw",
        duration: 1.5,
        ease: "expo.inOut",
      }
    );

    $gsap.fromTo(
      ".overlay",
      {
        opacity: 1,
      },
      {
        opacity: 0,
        duration: 1.5,
        ease: "expo.in",
        onComplete: () => {
          isOpened.value = false;
        },
      }
    );
  }
}

onMounted(() => {
  window.addEventListener("scroll", onScroll);
});
</script>
