<!--
  - SPDX-License-Identifier: AGPL-3.0-only
  -
  - This file is part of HarTex.
  -
  - HarTex
  - Copyright (c) 2021-2022 HarTex Project Developers
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
  <header class="navbar">
    <NuxtLink class="brand" to="/">HarTex</NuxtLink>
    <div v-if="isMobile">
      <Icon name="carbon:menu" />
    </div>
    <div v-else>
      <nav>
        <ul>
          <li><NuxtLink to="/">Documentation</NuxtLink></li>
        </ul>
        <ul>
          <li>
            <NuxtLink to="https://github.com/TeamHarTex/HarTex">Github</NuxtLink>
          </li>
        </ul>
        <ul>
          <li>
            <a v-on:click="">Login/Register</a>
          </li>
        </ul>
      </nav>
    </div>
  </header>
</template>

<script setup>
const isMobile = computed(() => {
  const isClientSide = process.client
  if (isClientSide)
    return window.screen.width < 1024
  return false
})
</script>

<style lang="postcss" scoped>
.navbar {
  @apply flex flex-row pt-16;
  @apply items-center justify-between;
}

.brand {
  @apply text-2xl sm:text-4xl text-secondary;
}

nav {
  @apply hidden sm:block;
}

nav ul {
  @apply inline-block mx-6 first:ml-0 last:mr-0;
}

svg {
  @apply block sm:hidden text-2xl;
}
</style>
