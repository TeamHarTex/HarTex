<template>
  <section>
    <div class="features">
      <LandingFeatureCard class="first" heading="Flexible configurations:">
        <div class="whitespace"></div>
        <ul>
          <li>Comprehensive API,</li>
          <li>Example,</li>
          <li>... and much more</li>
        </ul>
      </LandingFeatureCard>
      <LandingFeatureCard class="second" heading="Ticket system">
        <div class="i-carbon-ticket"></div>
      </LandingFeatureCard>
      <LandingFeatureCard class="third">
        <h2>
          Any questions?<br/>
          Join our
          <NuxtLink to="https://discord.com/invite/Xu8453VBAv"
                    class="inline-btn"
                    target="_blank">Discord
          </NuxtLink>
        </h2>
        <div class="i-carbon-share-knowledge"></div>
      </LandingFeatureCard>
      <LandingFeatureCard class="forth" heading="Leveling system">
        <div class="i-carbon-increase-level"></div>
      </LandingFeatureCard>
      <LandingFeatureCard class="fifth" heading="Starboard">
        <div class="i-carbon-star"></div>
      </LandingFeatureCard>
      <LandingFeatureCard class="sixth" heading="Robust moderation features:">
        <div class="whitespace"></div>
        <ul>
          <li>Infraction database,</li>
          <li>Example,</li>
          <li>... and much more</li>
        </ul>
      </LandingFeatureCard>
    </div>
  </section>
</template>

<style lang="postcss">
.features {
  @apply grid grid-cols-1 xl:grid-cols-4 gap-2xl;

  .first {
    grid-area: 1 / 1 / 2 / 3;
  }

  .third {
    @apply hidden xl:flex;
  }

  .sixth {
    grid-area: 2 / 3 / 3 / 5;
  }

  .first, .sixth {
    h2 {
      @apply text-2xl md:text-3xl 5xl:text-4xl;
    }

    ul {
      @apply flex-1 list-none text-2xl 5xl:text-3xl;
    }
  }

  .second, .third, .forth, .fifth {
    @apply items-center justify-center text-center;

    h2 {
      @apply text-xl md:text-2xl 5xl:text-3xl;
    }

    div {
      @apply text-5xl 5xl:text-6xl mt-8;
    }
  }

  @media only screen and (max-width: 1024px) {
    .first, .sixth {
      grid-area: auto;
    }
  }
}
</style>