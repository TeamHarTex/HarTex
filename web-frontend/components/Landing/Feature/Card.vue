<template>
  <div class="card">
    <h2>{{ heading }}</h2>
    <slot/>
  </div>
</template>

<style scoped lang="postcss">
.card {
  @apply flex flex-col h-80 rounded-lg;
  @apply bg-tertiary text-primary p-12;
}
</style>

<script setup lang="ts">
defineProps({
  heading: String
});
</script>