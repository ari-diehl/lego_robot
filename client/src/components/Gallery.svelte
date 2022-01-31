<script>
  import { onMount } from "svelte";

  import DrawingCard from "./DrawingCard.svelte";

  import { getDrawings } from "../api.js";

  let drawings = [];

  onMount(async () => {
    drawings = await getDrawings();
  });
</script>

<section class="flex flex-wrap">
  {#each drawings as src}
    <DrawingCard {src} />
  {/each}
</section>
