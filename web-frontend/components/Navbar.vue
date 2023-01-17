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
  <header class="navbar">
    <NuxtLink class="brand font-bold" to="/">HarTex</NuxtLink>
    <Icon name="carbon:menu" />
    <nav>
      <ul>
        <li><NuxtLink to="/">Documentation</NuxtLink></li>
      </ul>
      <ul>
        <li>
          <NuxtLink to="https://github.com/TeamHarTex/HarTex" target="_blank">
            GitHub
          </NuxtLink>
        </li>
      </ul>
      <ul>
        <li>
          <a v-on:click="">Login</a>
        </li>
      </ul>
    </nav>
  </header>
</template>

<style lang="postcss" scoped>
.navbar {
  @apply flex flex-row pt-16;
  @apply items-center justify-between;
  @apply mx-12 sm:mx-24 lg:mx-36 xl:mx-80;
}

.brand {
  @apply text-3xl sm:text-4xl text-secondary;
}

nav {
  @apply hidden sm:block;
}

nav ul {
  @apply inline-block mx-6 first:ml-0 last:mr-0;
}

svg {
  @apply visible sm:invisible text-2xl;
}
</style>
