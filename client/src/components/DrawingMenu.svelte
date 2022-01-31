<script>
  import { createEventDispatcher } from "svelte";

  import ContentSave from "svelte-material-icons/ContentSave.svelte";
  import Draw from "svelte-material-icons/Draw.svelte";
  import CloseCircleOutline from "svelte-material-icons/CloseCircleOutline.svelte";

  const dispatch = createEventDispatcher();
</script>

<menu class="m-0 py-2 px-0 flex">
  <button on:click={() => dispatch("clear")} class="bg-red-700 hover:bg-red-800"
    ><CloseCircleOutline /> <span>Clear</span></button
  >
  <button
    on:click={() => dispatch("save")}
    class="bg-blue-700 hover:bg-blue-800"
    ><ContentSave /> <span>Save</span></button
  >
  <button
    on:click={() => dispatch("draw")}
    class="bg-green-700 hover:bg-green-800"><Draw /> <span>Draw</span></button
  >
</menu>

<style>
  button {
    @apply mr-2 flex justify-center items-center rounded-sm w-20 text-white font-bold py-1;
  }

  span {
    @apply ml-1;
  }
</style>
