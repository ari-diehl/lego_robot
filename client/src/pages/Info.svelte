<script>
  import Page from "../components/Page.svelte";
</script>

<Page title="Info">
  <section class="p-5 rounded-sm overflow-hidden shadow-lg bg-white w-96">
    <h2 class="font-bold">Download Info-PDF</h2>
    <a
      class="inline-block w-40 text-center font-bold bg-green-700 hover:bg-green-800 p-2 mt-5 text-white rounded-sm"
      href="/info.pdf"
      download
    >
      Download
    </a>
  </section>
</Page>
