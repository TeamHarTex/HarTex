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
    <Icon name="carbon:menu" />
  </header>
</template>

<script setup>

</script>

<style lang="postcss" scoped>
.navbar {
  @apply flex flex-row pt-16;
  @apply items-center justify-between;
}

.brand {
  @apply text-2xl sm:text-4xl text-secondary;
}

nav {
  @apply hidden sm:block;
}

nav ul {
  @apply inline-block mx-6 first:ml-0 last:mr-0;
}

svg {
  @apply block sm:hidden text-2xl;
}
</style>
