<script>
  import { Router, Route } from "svelte-routing";

  import Home from "./pages/Home.svelte";
  import Drawings from "./pages/Drawings.svelte";
  import Controls from "./pages/Controls.svelte";
  import Info from "./pages/Info.svelte";
</script>

<Router url="">
  <Route path="/drawings" component={Drawings} />
  <Route path="/controls" component={Controls} />
  <Route path="/info" component={Info} />
  <Route path="/" component={Home} />
</Router>

<style global lang="postcss">
  @tailwind base;
  @tailwind components;
  @tailwind utilities;

  body {
    @apply bg-gray-100;
  }
</style>
