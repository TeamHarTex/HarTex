<!--
SPDX-License-Identifier: AGPL-3.0-only

This file is part of HarTex.

HarTex
Copyright (c) 2021-2023 HarTex Project Developers

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
  <div class="docs">
    <header>
      <NuxtLink class="brand">HarTex Documentation</NuxtLink>
    </header>
    <main>
      <aside>main index</aside>
      <article><ContentDoc/></article>
      <nav>article index</nav>
    </main>
  </div>
</template>

<style scoped lang="postcss">
header {
  line-height: 18px;

  @apply sticky top-0 bg-dark-900 p-5;
}

main {
  @apply grid;

  grid-template-columns: 24rem 1fr 24rem;
  grid-template-rows: auto 1fr auto;
  grid-template-areas: "aside article nav";
}

aside {
  grid-area: aside;

  @apply bg-dark-800;
}

article {
  grid-area: article;
  overflow: hidden;

  @apply bg-dark-600;
  @apply px-16 py-6;
}

article :deep(h1) {
  @apply pb-6;
  @apply text-[1.5vw];
}

nav {
  grid-area: nav;

  @apply bg-dark-400;
}

.brand {
  @apply font-bold text-[0.8vw];
}

.docs {
  @apply transition-all;
}
</style>
