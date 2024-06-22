import Lenis from "@studio-freight/lenis";

export const useSmoothScroll = () => {
  const { $gsap, $ScrollTrigger } = useNuxtApp();

  const lenis = new Lenis({
    duration: 1.5,
  });

  lenis.on("scroll", () => {
    $ScrollTrigger.update();
  });

  $gsap.ticker.add((time) => {
    lenis.raf(time * 1000);
  });

  $gsap.ticker.lagSmoothing(0);
};
