import * as THREE from "https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.module.js";

let scene, camera, renderer;
let mazeGroup, pathGroup, pin;

const cellSize = 1;
const height = 0.5;

init();
animate();

function init() {
  // Scene setup
  scene = new THREE.Scene();
  scene.background = new THREE.Color(0xf0f0f0);

  // Camera setup
  camera = new THREE.PerspectiveCamera(
    75,
    window.innerWidth / window.innerHeight,
    0.1,
    1000,
  );
  camera.position.set(50, 75, 150); // Adjust the camera height to tilt the maze upward
  camera.lookAt(50, 0, 50);

  // Renderer setup
  renderer = new THREE.WebGLRenderer();
  renderer.setSize(window.innerWidth, window.innerHeight);
  document.body.appendChild(renderer.domElement);

  // Lighting setup
  const ambientLight = new THREE.AmbientLight(0x404040); // Soft white light
  scene.add(ambientLight);

  const directionalLight = new THREE.DirectionalLight(0xffffff, 0.5);
  directionalLight.position.set(1, 1, 1).normalize();
  scene.add(directionalLight);

  // Initialize groups
  mazeGroup = new THREE.Group();
  pathGroup = new THREE.Group();
  scene.add(mazeGroup);
  scene.add(pathGroup);

  // Load the Google Maps pin image as a texture
  const textureLoader = new THREE.TextureLoader();
  textureLoader.load("./red-pin-pin.png", function (texture) {
    const pinMaterial = new THREE.SpriteMaterial({ map: texture });
    pin = new THREE.Sprite(pinMaterial);
    pin.scale.set(4, 4, 1); // Scale down by 25% (was 4, 4, 1)
    pin.position.set(0, height, 0);
    scene.add(pin);
  });

  startPathfinding();
}

function drawMaze(matrix) {
  mazeGroup.clear();
  for (let row = 0; row < matrix.length; row++) {
    for (let col = 0; col < matrix[row].length; col++) {
      if (matrix[row][col] === 1) {
        const wallGeometry = new THREE.BoxGeometry(cellSize, height, cellSize);
        const wallMaterial = new THREE.MeshBasicMaterial({ color: 0x333333 });
        const wall = new THREE.Mesh(wallGeometry, wallMaterial);
        wall.position.set(col, height / 2, row);
        mazeGroup.add(wall);
      }
    }
  }
}

function animatePath(path) {
  pathGroup.clear();
  let index = 0;
  const dotGeometry = new THREE.SphereGeometry(cellSize / 2, 32, 32);
  const dotMaterial = new THREE.MeshBasicMaterial({ color: 0xffd700 }); // Yellow/Gold color

  function step() {
    if (index < path.length) {
      const [row, col] = path[index];

      // Move the pin
      if (pin) {
        pin.position.set(col, height, row);
        // Adjust the camera to follow the pin, with a tilt for better visibility
        camera.position.set(col, height * 25, row + 20);
        camera.lookAt(col, 0, row);
      }

      // Leave a yellow/gold dot behind
      const dot = new THREE.Mesh(dotGeometry, dotMaterial);
      dot.position.set(col, height / 2, row);
      pathGroup.add(dot);

      index++;
      setTimeout(step, 50); // Delay for visualizing the animation
    }
  }
  step();
}

async function startPathfinding() {
  const response = await fetch("/path", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ start: [0, 0], goal: [99, 99] }),
  });
  const data = await response.json();
  if (response.ok) {
    drawMaze(data.matrix); // Draw the maze using 3D walls
    animatePath(data.path); // Animate the path in 3D
  } else {
    alert("Path not found");
  }
}

function animate() {
  requestAnimationFrame(animate);
  renderer.render(scene, camera);
}
