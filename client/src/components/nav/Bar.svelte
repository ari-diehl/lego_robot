<script>
  import PaletteOutline from "svelte-material-icons/PaletteOutline.svelte";
  import FileImageOutline from "svelte-material-icons/FileImageOutline.svelte";
  import CameraControl from "svelte-material-icons/CameraControl.svelte";
  import InformationOutline from "svelte-material-icons/InformationOutline.svelte";

  import Logo from "./Logo.svelte";
  import Item from "./Item.svelte";

  const size = "2.5em";
</script>

<nav
  style="min-height:500px; "
  class="fixed w-16 h-screen flex flex-col justify-center items-center"
>
  <Logo />
  <Item to="/"><PaletteOutline {size} /></Item>
  <Item to="/drawings"><FileImageOutline {size} /></Item>
  <Item to="/controls"><CameraControl {size} /></Item>
  <Item to="/info"><InformationOutline {size} /></Item>
</nav>
