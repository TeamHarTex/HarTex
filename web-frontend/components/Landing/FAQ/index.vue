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
  <section class="faq">
    <div class="left">
      <h2 class="subheading">FAQ</h2>
      <div class="whitespace"></div>
      <p>
        If your question is not listed, you may want to search through the
        GitHub issues page, or even message us on Discord.
      </p>
    </div>
    <div class="right">
      <LandingFAQAccordion question="Example question" answer="test" />
      <LandingFAQAccordion question="Example question" answer="test" />
      <LandingFAQAccordion question="Example question" answer="test" />
    </div>
  </section>
</template>

<style scoped lang="postcss">
.faq {
  @apply flex;

  .left {
    @apply w-35% pr-10;

    h2 {
      @apply text-5xl text-secondary;
    }

    p {
      @apply text-xl;
    }
  }

  .right {
    @apply flex-1 pl-10;

    .accordion {
      @apply my-10;

      &:first-child {
        @apply mt-0;
      }

      &:last-child {
        @apply mb-0;
      }
    }
  }
}
</style>
