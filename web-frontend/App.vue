<!--
  - SPDX-License-Identifier: AGPL-3.0-only
  -
  - This file is part of HarTex.
  -
  - HarTex
  - Copyright (c) 2021-2023 HarTex Project Developers
  -
  - HarTex is free software; you can redistribute it and/or modify
  - it under the terms of the GNU Affero General Public License as published by
  - the Free Software Foundation; either version 3 of the License, or
  - (at your option) any later version.
  -
  - HarTex is distributed in the hope that it will be useful,
  - but WITHOUT ANY WARRANTY; without even the implied warranty of
  - MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
  - GNU Affero General Public License for more details.
  -
  - You should have received a copy of the GNU Affero General Public License along
  - with HarTex. If not, see <https://www.gnu.org/licenses/>.
  -->

<template>
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>

<script setup></script>

<style lang="postcss">
a {
  @apply font-semibold text-tertiary hover:text-secondary;
  @apply transition-colors cursor-pointer;
}

body {
  @apply bg-primary text-secondary mb-52;

  font-family: "Inter", sans-serif;
}

br {
  @apply select-none;
}

button {
  @apply px-12 py-3 cursor-pointer;
  @apply bg-blue-800 font-semibold;
  @apply border border-blue-800 rounded-3xl;

  @apply transition hover:bg-blue-900 hover:border-blue-900;
}

svg {
  @apply cursor-pointer;
}

.whitespace {
  @apply my-4;
}

::selection {
  @apply bg-blue-700 text-white;
}
</style>
