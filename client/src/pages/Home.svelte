<script>
  import Page from "../components/Page.svelte";
  import DrawingArea from "../components/DrawingArea.svelte";
  import DrawingMenu from "../components/DrawingMenu.svelte";

  let drawingArea;
</script>

<Page title="Plott3rPaint">
  <DrawingMenu
    on:clear={() => drawingArea.clear()}
    on:draw={() => drawingArea.draw()}
    on:save={() => drawingArea.save()}
  />
  <DrawingArea bind:this={drawingArea} />
</Page>
