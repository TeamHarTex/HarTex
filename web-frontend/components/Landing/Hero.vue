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
  <header>
    <h1>
      Discord server management,<br/>
      <span>reimagined</span>.
    </h1>
    <NuxtLink class="primary-btn">Add to Discord</NuxtLink>
  </header>
</template>

<style scoped lang="postcss">
header {
  @apply h-[95vh] flex flex-col items-center justify-center;

  h1 {
    @apply text-5xl md:text-6xl 5xl:text-8xl font-500 text-center;

    span {
      @apply text-secondary font-serif;
    }
  }

  .primary-btn {
    @apply mt-20 cursor-not-allowed opacity-50;
  }

  .primary-btn:hover {
    @apply bg-primary text-tertiary;
  }
}
</style>