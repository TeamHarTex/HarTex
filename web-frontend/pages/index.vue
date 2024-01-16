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
  <section>
    <div class="features">
      <div class="feature first">
        <h2>Flexible configurations:</h2>
        <div class="whitespace"></div>
        <ul>
          <li>Comprehensive API,</li>
          <li>Example,</li>
          <li>... and much more</li>
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
          <li>Infraction database,</li>
          <li>Example,</li>
          <li>... and much more</li>
        </ul>
      </div>
    </div>
  </section>
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

.features {
  @apply grid grid-cols-1 xl:grid-cols-4 gap-2xl;

  .feature {
    @apply flex flex-col h-80 rounded-lg;
    @apply bg-tertiary text-primary p-12;
  }

  .first {
    grid-area: 1 / 1 / 2 / 3;
  }

  .third {
    @apply hidden xl:flex;
  }

  .sixth {
    grid-area: 2 / 3 / 3 / 5;
  }

  .first, .sixth {
    h2 {
      @apply text-2xl md:text-3xl 5xl:text-4xl;
    }

    ul {
      @apply flex-1 list-none text-2xl 5xl:text-3xl;
    }
  }

  .second, .third, .forth, .fifth {
    @apply items-center justify-center text-center;

    h2 {
      @apply text-xl md:text-2xl 5xl:text-3xl;
    }

    div {
      @apply text-5xl 5xl:text-8xl mt-8;
    }
  }

  @media only screen and (max-width: 1024px) {
    .first, .sixth {
      grid-area: auto;
    }
  }
}

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