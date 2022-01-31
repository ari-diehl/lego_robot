<script>
  import ArrowLeftBold from "svelte-material-icons/ArrowLeftBold.svelte";
  import ArrowRightBold from "svelte-material-icons/ArrowRightBold.svelte";
  import ArrowUpBold from "svelte-material-icons/ArrowUpBold.svelte";
  import ArrowDownBold from "svelte-material-icons/ArrowDownBold.svelte";
  import Stop from "svelte-material-icons/Stop.svelte";
  import FileOutline from "svelte-material-icons/FileOutline.svelte";
  import Pencil from "svelte-material-icons/Pencil.svelte";

  import { postControls } from "../api.js";

  let size = "1.5em";
  let blueButtonClass = "bg-blue-700 hover:bg-blue-800";
  let redButtonClass = "bg-red-700 hover:bg-red-800";
</script>

<section
  class="p-5 rounded-sm overflow-hidden shadow-lg bg-white flex w-96 justify-evenly"
>
  <div class="flex flex-col items-center">
    <div class="mb-10 flex items-center">
      <Pencil {size} /><span class="ml-1 font-bold">Pen</span>
    </div>
    <button class={blueButtonClass} on:click={() => postControls("pen", "up")}
      ><ArrowUpBold {size} /></button
    >
    <div class="flex">
      <button
        class={blueButtonClass}
        on:click={() => postControls("pen", "left")}
        ><ArrowLeftBold {size} /></button
      >
      <button
        class={redButtonClass}
        on:click={() => postControls("pen", "stop")}><Stop {size} /></button
      >
      <button
        class={blueButtonClass}
        on:click={() => postControls("pen", "right")}
        ><ArrowRightBold {size} /></button
      >
    </div>
    <button class={blueButtonClass} on:click={() => postControls("pen", "down")}
      ><ArrowDownBold {size} /></button
    >
  </div>
  <div class="flex flex-col items-center">
    <div class="mb-10 flex items-center">
      <FileOutline {size} /><span class="ml-1 font-bold">Paper</span>
    </div>
    <button class={blueButtonClass} on:click={() => postControls("paper", "in")}
      ><ArrowUpBold {size} /></button
    >
    <button
      class={redButtonClass}
      on:click={() => postControls("paper", "stop")}><Stop {size} /></button
    >
    <button
      class={blueButtonClass}
      on:click={() => postControls("paper", "out")}
      ><ArrowDownBold {size} /></button
    >
  </div>
</section>

<style>
  button {
    @apply text-white p-2 m-1 rounded-sm;
  }
</style>
