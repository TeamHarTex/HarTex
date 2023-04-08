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
      <button>Get Started</button>
      <div class="whitespace block sm:hidden" />
      <button @click="toWhyHarTex()">Learn More</button>
    </div>
  </Section>
  <Section class="why" id="why-hartex">
    <h3>Why HarTex?</h3>
    <div class="whitespace" />
    <p>HarTex caters...</p>
    <hr>
  </Section>
</template>

<script lang="ts">
export default {
  methods: {
    toWhyHarTex() {
        document.querySelector("#why-hartex")?.scrollIntoView({ behavior: "smooth" });
    }
  }
}
</script>

<style lang="postcss" scoped>
button {
  @apply text-lg;
  @apply sm:mr-4;
}

hr {
  @apply text-gray-600 opacity-50;
  @apply w-[100%];
}

.hero {
  @apply flex flex-col;
  @apply text-center justify-center items-center;
  @apply h-[95vh];
}

.hero h2 {
  @apply text-4xl sm:text-5xl xl:text-6xl font-bold tracking-tight;
}

.hero p {
  @apply text-lg sm:text-xl text-tertiary;
}

.why {
  @apply flex flex-col;
  @apply text-center items-center;
}

.why h3 {
  @apply text-2xl sm:text-3xl xl:text-4xl font-bold tracking-tight;
}

.why p {
    @apply text-lg sm:text-xl text-tertiary;
    @apply pb-4 mb-4;
}
</style>
