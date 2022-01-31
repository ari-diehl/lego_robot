const BASE_URL = "http://ev3dev:8080/api";

async function get(to) {
  return await fetch(BASE_URL.concat(to));
}

async function post(to, body) {
  return await fetch(BASE_URL.concat(to), {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(body),
  });
}

export async function postDraw(drawingInstructions) {
  return await post("/draw", { drawing_instructions: drawingInstructions });
}

export async function postControls(component, command) {
  return await post("/controls", { component, command });
}

export async function getDrawings() {
  const response = await (await get("/drawings")).json();

  const drawings = [];

  for (let drawing of response) {
    drawings.push(drawing);
  }

  return drawings;
}

export async function postDrawings(name, drawing, drawingInstructions) {
  return await post("/drawings", { name, drawing, drawing_instructions: JSON.stringify(drawingInstructions) });
}

export async function getDrawingInstructions(name) {
  return await get("/drawing_instructions?name=".concat(name))
}
