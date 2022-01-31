<script>
  import { Link } from "svelte-routing";

  export let to;
</script>

<Link class="p-5 text-white" {to}><slot /></Link>
