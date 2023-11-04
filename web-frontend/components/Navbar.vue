<template>
  <div class="navbar">
    <NuxtLink class="title" to="/">
      hartex/<Transition @enter="onEnter" @leave="onLeave" :css="false">
        <span v-if="suffix()" :key="suffix()">{{ suffix() }}</span>
      </Transition>
    </NuxtLink>
    <NuxtLink to="https://github.com/TeamHarTex/HarTex" target="_blank">
      <div class="i-carbon-logo-github"></div>
    </NuxtLink>
  </div>
</template>

<style scoped lang="postcss">
.navbar {
  @apply sticky top-0 opacity-0;
  @apply px-18 py-12 text-3xl;
  @apply flex justify-between items-center;
}

.navbar .title {
  @apply font-600;
}

.navbar span {
  @apply inline-block;
}

@screen lt-md {
  .navbar {
    @apply px-12;
  }
}
</style>

<script setup lang="ts">
const { $gsap } = useNuxtApp();

const route = useRoute();

function suffix() {
  switch (route.name) {
    case "blog-slug": {
      return "blog";
    }

    case "docs-slug": {
      return "docs";
    }
  }
}

function onEnter(el: any, done: any) {
  $gsap.fromTo(
    el,
    {
      opacity: 0,
      y: -30,
    },
    {
      opacity: 1,
      y: 0,
      duration: 0.3,
      delay: 0.3,
      onComplete: done,
    }
  );
}

function onLeave(el: any, done: any) {
  $gsap.to(el, {
    opacity: 0,
    duration: 0.3,
    onComplete: done,
  });
}

onMounted(() => {
  $gsap.fromTo(
    ".navbar",
    {
      y: -30,
    },
    {
      opacity: 1,
      y: 0,
      duration: 0.3,
      delay: 0.6,
    }
  );
});
</script>
