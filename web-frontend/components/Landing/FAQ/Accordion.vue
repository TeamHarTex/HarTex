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
  <div class="accordion">
    <button @click="toggle">
      <div :class="{ 'rotate-45': isOpened }" class="i-carbon-add"></div>
      {{ props.question }}
    </button>
    <div ref="content">
      <p :class="{ hidden: !isOpened }">{{ props.answer }}</p>
    </div>
  </div>
</template>

<style scoped lang="postcss">
.accordion {
  @apply border-solid border-1 border-tertiary border-rd;

  button {
    @apply flex items-center;
    @apply text-left text-lg font-500;
    @apply w-full p-6;

    div {
      @apply transition-transform text-2xl mr-4;
    }

    &:hover {
      @apply border-secondary;
    }
  }

  p {
    @apply pl-6 pb-6 text-lg;
  }
}
</style>

<script setup lang="ts">
const { $gsap } = useNuxtApp();

const props = defineProps<{
  question: string;
  answer: string;
}>();

const content: Ref<HTMLDivElement | null> = ref(null);
const isOpened = ref(false);

function toggle() {
  isOpened.value = !isOpened.value;

  if (isOpened.value) {
    $gsap.fromTo(
      content.value,
      { height: 0 },
      { height: "auto", duration: 0.25, ease: "expo.out" }
    );
  } else {
    const height = content.value?.offsetHeight;
    content.value!.style.height = `${height}px`;

    $gsap.to(content.value, {
      height: 0,
      duration: 0.25,
      ease: "expo.out",
    });
  }
}
</script>
