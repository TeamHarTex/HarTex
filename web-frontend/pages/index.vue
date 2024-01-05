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
      Discord servers' management,<br/>
      <span>reimagined</span>.
    </h1>
  </header>
  <section>
    <div class="features">
      <div class="feature first">
      </div>
      <div class="feature second">
      </div>
      <div class="feature third">
      </div>
      <div class="feature forth">
      </div>
      <div class="feature fifth">
      </div>
      <div class="feature sixth">
      </div>
    </div>
  </section>
</template>

<style scoped lang="postcss">
header {
  @apply h-4xl flex items-center justify-center;
}

header h1 {
  @apply text-6xl font-500 text-center;
}

header h1 span {
  @apply text-secondary font-serif;
}

.features {
  @apply grid grid-cols-4 gap-2xl;
}

.features .feature {
  @apply flex h-70 rounded-lg;
  @apply bg-tertiary text-primary;
}

.first {
  grid-area: 1 / 1 / 2 / 3;
}

.second {
  grid-area: 1 / 3 / 2 / 4;
}

.third {
  grid-area: 1 / 4 / 2 / 5;
}

.forth {
  grid-area: 2 / 1 / 3 / 2;
}

.fifth {
  grid-area: 2 / 2 / 3 / 3;
}

.sixth {
  grid-area: 2 / 3 / 3 / 5;
}
</style>
