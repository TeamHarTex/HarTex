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
  <Section class="hero">
    <h2>
      The speed-of-thought<br class="hidden sm:block" />
      server management bot for Discord.
    </h2>
    <div class="whitespace" />
    <p>
      Made with love by Team HarTex.
    </p>
    <div class="whitespace" />
    <div>
      <button class="sm:mr-4">Get Started</button>
      <div class="whitespace block sm:hidden" />
      <button>Learn More</button>
    </div>
  </Section>
  <Section></Section>
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
