<script>
  import { Link } from "svelte-routing";

  export let src;
</script>

<article
  title={src.slice(0, -4)}
  class="p-5 rounded-sm overflow-hidden shadow-lg bg-white m-3 hover:scale-110 hover:cursor-pointer"
>
  <Link to={"/?name=".concat(src.slice(0, -4))}
    ><img width="300" src={"/drawings/".concat(src)} alt="User drawing" /></Link
  >
</article>
