<template>
  <div class="modal">
    <h2>{{ props.title }}</h2>
    <slot />
  </div>
</template>

<script setup>
const props = defineProps({
  title: String,
});
</script>
