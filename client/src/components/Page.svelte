<script>
  import NavBar from "./nav/Bar.svelte";
  export let title;
</script>

<NavBar />
<main class="ml-24 pb-10">
  <h2 class="font-bold text-3xl pt-7 pb-4 text-neutral-900">
    {title}
  </h2>
  <slot />
</main>
