<script>
  import { onMount } from "svelte";

  import { postDraw, postDrawings, getDrawingInstructions } from "../api.js";

  let drawingInstructions = [];
  let currentDrawingInstructions = [];

  let canvas;
  let context;

  let isMousedown = false;

  let previousX = 0;
  let previousY = 0;
  let currentX = 0;
  let currentY = 0;

  function setCoordinates(event) {
    previousX = currentX;
    previousY = currentY;

    currentX = event.clientX - canvas.offsetLeft;
    currentY = event.clientY - canvas.offsetTop + window.scrollY;
  }

  function handleMouseEvent(event) {
    switch (event.type) {
      case "mousedown":
        setCoordinates(event);

        context.beginPath();
        context.fillRect(
          currentX,
          currentY,
          context.lineWidth,
          context.lineWidth
        );
        context.closePath();

        isMousedown = true;

        currentDrawingInstructions = [
          ...currentDrawingInstructions,
          [currentX - previousX, currentY - previousY],
        ];

        break;
      case "mousemove":
        if (isMousedown) {
          setCoordinates(event);

          context.beginPath();
          context.moveTo(previousX, previousY);
          context.lineTo(currentX, currentY);
          context.stroke();
          context.closePath();

          currentDrawingInstructions = [
            ...currentDrawingInstructions,
            [currentX - previousX, currentY - previousY],
          ];
        }

        break;
      case "mouseup":
        isMousedown = false;

        drawingInstructions = [
          ...drawingInstructions,
          currentDrawingInstructions,
        ];
        currentDrawingInstructions = [];

        break;
      case "mouseout":
        isMousedown = false;

        break;
    }
  }

  export function clear() {
    context.clearRect(0, 0, canvas.width, canvas.height);
    drawingInstructions = [];
    currentDrawingInstructions = [];
  }

  export function draw() {
    postDraw(drawingInstructions);
  }

  export function save() {
    const name = prompt("Please name your drawing:");
    if (name) {
      postDrawings(name, canvas.toDataURL().slice(22), drawingInstructions);
    }
  }

  onMount(async () => {
    context = canvas.getContext("2d");
    context.fillStyle = "black";
    context.lineWidth = 2;

    const name = new URLSearchParams(window.location.search).get("name");

    if (name) {
      canvas.style.display = "none";
      const drawing = new Image();
      drawing.src = "drawings/".concat(name).concat(".png");

      context.drawImage(drawing, 0, 0);

      drawingInstructions = await (await getDrawingInstructions(name)).json();
      currentDrawingInstructions = [];

      canvas.style.display = "block";
    }
  });
</script>

<canvas
  bind:this={canvas}
  on:mousedown={handleMouseEvent}
  on:mousemove={handleMouseEvent}
  on:mouseup={handleMouseEvent}
  on:mouseout={handleMouseEvent}
  class="mb-10 bg-white"
  width="1100"
  height="800"
/>
