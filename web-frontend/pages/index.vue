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
  <section class="hero">
    <h2>
      The speed of thought<br class="hidden sm:block" />
      Discord management bot.
    </h2>
    <br />
    <p>
      Made with love by Team HarTex.
    </p>
    <br />
    <button>Get Started</button>
  </section>
</template>

<style lang="postcss" scoped>
.hero {
  @apply flex flex-col;
  @apply text-center justify-center items-center;
}

.hero h2 {
  @apply text-4xl sm:text-5xl xl:text-6xl font-bold tracking-tight;
}

.hero p {
  @apply text-sm sm:text-lg text-tertiary;
}
</style>
