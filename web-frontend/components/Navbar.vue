<template>
  <header class="navbar">
    <NuxtLink class="brand" to="/">hartex.</NuxtLink>
    <nav>
      <ul>
        <li><NuxtLink to="/">Docs</NuxtLink></li>
      </ul>
      <ul>
        <li>
          <NuxtLink to="https://github.com/TeamHarTex/HarTex">Github</NuxtLink>
        </li>
      </ul>
      <ul>
        <li>
          <a v-on:click="">Login/Register</a>
        </li>
      </ul>
    </nav>
  </header>
</template>

<script setup>

</script>

<style lang="postcss" scoped>
.navbar {
  @apply flex flex-row pb-4;
  @apply items-center justify-between;
}

.brand {
  @apply text-4xl text-secondary;
}

nav ul {
  @apply inline-block mx-6 first:ml-0 last:mr-0;
}
</style>
