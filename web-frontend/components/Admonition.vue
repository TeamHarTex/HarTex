<!--
  - SPDX-License-Identifier: AGPL-3.0-only
  -
  - This file is part of HarTex.
  -
  - HarTex
  - Copyright (c) 2021-2024 HarTex Project Developers
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
  <div :class="[type]">
    <div class="heading">
      <svg class="icon" xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 512 512"><path d="M256 32c14.2 0 27.3 7.5 34.5 19.8l216 368c7.3 12.4 7.3 27.7 .2 40.1S486.3 480 472 480H40c-14.3 0-27.6-7.7-34.7-20.1s-7-27.8 .2-40.1l216-368C228.7 39.5 241.8 32 256 32zm0 128c-13.3 0-24 10.7-24 24V296c0 13.3 10.7 24 24 24s24-10.7 24-24V184c0-13.3-10.7-24-24-24zm32 224a32 32 0 1 0 -64 0 32 32 0 1 0 64 0z"/></svg>
      {{title}}
    </div>
    <ContentSlot :use="$slots.default" unwrap="p" />
  </div>
</template>

<script setup lang="ts">
defineProps<{
  title: string,
  type: string
}>()
</script>

<style>
.icon {
  display: inline-block;
  margin-right: .3rem;
  vertical-align: middle;
}

.warning .heading .icon {
  fill: #FFF7E6;
}

.warning {
  background-color: #FFCC6655;
  border: 0;
  border-left: 5px solid #FFCC66;
  border-radius: 7px;

  @apply p-5 my-4;
}

.warning .heading {
  box-sizing: border-box;
  margin-bottom: .3rem;

  color: #FFF7E6;

  @apply font-bold;
}

</style>
