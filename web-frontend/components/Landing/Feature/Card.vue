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
  <div class="card" ref="card">
    <h2>{{ heading }}</h2>
    <slot/>
  </div>
</template>

<style scoped lang="postcss">
.card {
  @apply flex flex-col h-80 rounded-lg;
  @apply bg-tertiary text-primary p-12;

  transform: v-bind(cardTransform);
  transition: transform 0.5s ease-out;
}
</style>

<script setup lang="ts">
defineProps({
  heading: String
});

const card = ref(null);

const {elementX, elementY, elementHeight, elementWidth, isOutside} = useMouseInElement(card);

const cardTransform = computed(() => {
  const maxRotation = 5;

  const rX = (maxRotation / 2 - (elementY.value / elementHeight.value) * maxRotation).toFixed(2);
  const rY = (maxRotation / 2 - (elementX.value / elementWidth.value) * maxRotation).toFixed(2);

  return isOutside.value ? '' : `perspective(${elementWidth.value}px) rotateX(${rX}deg) rotateY(${rY}deg)`;
});
</script>