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
  </header>
  <section>
    <div class="features">
      <div class="feature first">
        <h2>Flexible configurations:</h2>
        <div class="whitespace"></div>
        <ul>
          <li>...</li>
          <li>...</li>
          <li>...</li>
        </ul>
      </div>
      <div class="feature second">
        <h2>Ticket system</h2>
        <div class="i-carbon-ticket"></div>
      </div>
      <div class="feature third">
        <h2>
          Any questions?<br />
          Join our
          <NuxtLink to="https://discord.com/invite/Xu8453VBAv"
            class="inline-btn"
            target="_blank">Discord</NuxtLink>!
        </h2>
        <div class="i-carbon-share-knowledge"></div>
      </div>
      <div class="feature forth">
        <h2>Leveling system</h2>
        <div class="i-carbon-increase-level"></div>
      </div>
      <div class="feature fifth">
        <h2>Starboard</h2>
        <div class="i-carbon-star"></div>
      </div>
      <div class="feature sixth">
        <h2>Robust moderation features:</h2>
        <div class="whitespace"></div>
        <ul>
          <li>Infraction database</li>
          <li>...</li>
          <li>...</li>
        </ul>
      </div>
    </div>
  </section>
</template>

<style scoped lang="postcss">
header {
  @apply h-4xl flex items-center justify-center;

  h1 {
    @apply text-6xl font-500 text-center;

    span {
      @apply text-secondary font-serif;
    }
  }
}

.features {
  @apply grid grid-cols-4 gap-2xl;

  .feature {
    @apply flex flex-col h-80 rounded-lg;
    @apply bg-tertiary text-primary p-12;
  }

  .first {
    grid-area: 1 / 1 / 2 / 3;
  }

  .sixth {
    grid-area: 2 / 3 / 3 / 5;
  }

  .first, .sixth {
    h2 {
      @apply text-3xl;
    }

    ul {
      @apply flex-1 list-none text-2xl;
    }
  }

  .second, .third, .forth, .fifth {
    @apply items-center justify-center text-center;

    div {
      @apply text-5xl mt-8;
    }
  }
}
</style>