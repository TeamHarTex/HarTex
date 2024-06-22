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
    <NuxtPage />
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
  @apply cursor-pointer decoration-none select-none;
  @apply text-tertiary text-center transition-colors duration-300;

  &:hover {
    @apply text-secondary;
  }
}

body {
  @apply bg-primary text-tertiary;
}

main,
nav,
footer {
  @apply px-10 sm:px-20 md:px-30 lg:px-40 xl:px-60 2xl:px-80;
}

main {
  @apply min-h-screen;
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
  @apply mx-6 px-10 py-5;
  @apply font-600 text-lg md:text-xl 5xl:text-2xl;
  @apply border-solid border-rd-[1000px] border-2;
  @apply bg-secondary text-primary!;

  &:hover {
    @apply bg-primary text-secondary!;
  }
}

.inline-btn {
  @apply text-primary underline;
}

.whitespace {
  @apply py-4;
}

html.lenis,
html.lenis body {
  height: auto;
}

.lenis.lenis-smooth {
  scroll-behavior: auto !important;
}

.lenis.lenis-smooth [data-lenis-prevent] {
  overscroll-behavior: contain;
}

.lenis.lenis-stopped {
  overflow: hidden;
}

.lenis.lenis-smooth iframe {
  pointer-events: none;
}
</style>
