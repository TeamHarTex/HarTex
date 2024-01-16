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
  <NuxtLayout>
    <NuxtPage/>
  </NuxtLayout>
</template>

<style lang="postcss">
::selection {
  @apply bg-secondary text-primary;
}

* {
  @apply m-0 p-0 box-border;
  @apply font-sans outline-none;
}

a {
  @apply decoration-none select-none;
  @apply text-tertiary text-center transition-colors;
}

a:hover {
  @apply text-secondary;
}

body {
  @apply bg-primary text-tertiary;
}

main, nav, footer {
  @apply px-10 sm:px-20 md:px-30 lg:px-40 xl:px-60 2xl:px-80;
}

section {
  @apply min-h-xl my-9xl;
}

section:first-of-type {
  @apply mt-0;
}

section:last-of-type {
  @apply mb-0;
}

.primary-btn {
  @apply mx-6 px-4 py-2 text-lg md:text-2xl 5xl:text-3xl;
  @apply border-solid border-rd border-1;
}

.primary-btn:hover {
  @apply bg-secondary text-primary;
}

.inline-btn {
  @apply text-primary underline;
}

.whitespace {
  @apply py-4;
}
</style>

<script setup lang="ts">
import Lenis from '@studio-freight/lenis'

function initLenis() {
  const lenis = new Lenis();

  function raf(time: any) {
    lenis.raf(time);

    requestAnimationFrame(raf);
  }

  requestAnimationFrame(raf);
}

onMounted(() => {
  initLenis();
})
</script>