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
  <section class="contribute">
    <h2>Want to contribute to or support HarTex?</h2>
    <div class="whitespace"></div>
    <div class="links">
      <NuxtLink class="primary-btn" to="https://github.com/TeamHarTex/HarTex" target="_blank">
        GitHub
      </NuxtLink>
      <NuxtLink class="primary-btn" to="https://opencollective.com/HarTex" target="_blank">
        Open Collective
      </NuxtLink>
    </div>
  </section>
</template>

<style scoped lang="postcss">
.contribute {
  @apply flex flex-col items-center justify-center;

  h2 {
    @apply text-center text-2xl md:text-3xl 5xl:text-5xl;
  }

  .links {
    @apply flex flex-col md:flex-row;

    a {
      @apply my-4 md:my-0;
    }

    a:first-child {
      @apply md:ml-0;
    }

    a:last-child {
      @apply mb-0 md:mr-0;
    }
  }
}
</style>