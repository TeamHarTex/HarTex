<!--
SPDX-License-Identifier: AGPL-3.0-only

This file is part of HarTex.

HarTex
Copyright (c) 2021-2023 HarTex Project Developers

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
    <div class="flex">
        <div class="half menu">
            <h1 class="title">HarTex Interactive Docs</h1>
            <input class="searchbar" placeholder="Type to search..." />
            <p class="browse">or <button>browse through a list</button>.</p>
        </div>
        <div class="half content">
            <p>markdown content stuff...</p>
        </div>
    </div>
</template>

<style scoped lang="postcss">
.half {
    @apply flex flex-1 flex-col;
    @apply h-screen justify-center items-center;
}

.menu {
    @apply bg-main-background;
}

.title {
    @apply text-4xl mb-8;
}

.searchbar {
    @apply w-[60%];
}

.browse {
    @apply text-xl mt-8;
}

.content {
    @apply bg-gray-950;
}
</style>
