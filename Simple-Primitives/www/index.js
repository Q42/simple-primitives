// ==== STARTUP
import * as wasm from '../pkg/simple_primitives';

const webClient = wasm.WebClient.new();
webClient.initCallBacks();

// ==== INPUTS
// These are all the inputs from the editor screen, we want to set them here because the are used in different parts of this file.
const inputShape = document.getElementById('dropdown-shapes');

const inputScaleX = document.getElementById('inputScaleX');
const inputScaleY = document.getElementById('inputScaleY');
const inputScaleZ = document.getElementById('inputScaleZ');

const inputSubdivisions = document.getElementById('inputSubdivisions');
const inputSides = document.getElementById('inputSides');
const inputRadius = document.getElementById('inputRadius');
const inputInnerRadius = document.getElementById('inputInnerRadius');
const inputOuterRadius = document.getElementById('inputOuterRadius');

// ==== GENERATE
const btnGenerate = document.getElementById('btnGenerate');

btnGenerate.onclick = function() {
  generate();
}

// Here we determine which shape we need to generate and we pass the right settings to the generateShape function.
function generate() {
  switch(inputShape.value) {
    case 'plane':
      generateShape([inputSubdivisions.value]);
      break;
    case 'disk':
      generateShape([inputSides.value, inputRadius.value]);
      break;
    case 'cube':
      generateShape([inputSubdivisions.value]);
      break;
    case 'sphere':
      generateShape([inputSubdivisions.value]);
      break;
    case 'cylinder':
      generateShape([inputSides.value, inputRadius.value]);
      break;
    case 'tube':
      generateShape([inputSides.value, inputInnerRadius.value, inputOuterRadius.value]);
    }

  webClient.drawScene();
}

function generateShape(args) {
  webClient.generate(inputShape.selectedIndex, [inputScaleX.value, inputScaleY.value, inputScaleZ.value], args);
}

// ==== EXPORT
const btnExport = document.getElementById('btnExport');

let fileContents;

btnExport.onclick = function () {
  switch(inputShape.value) {
    case 'plane':
      fileContents = exportShape([inputSubdivisions.value]);
      break;
    case 'disk':
      fileContents = exportShape([inputSides.value, inputRadius.value]);
      break;
    case 'cube':
      fileContents = exportShape([inputSubdivisions.value]);
      break;
    case 'sphere':
      fileContents = exportShape([inputSubdivisions.value]);
      break;
    case 'cylinder':
      fileContents = exportShape([inputSides.value, inputRadius.value]);
      break;
    case 'tube':
      fileContents = exportShape([inputSides.value, inputInnerRadius.value, inputOuterRadius.value]);
  }
  
  download("export.obj", fileContents);
}

function exportShape(args) {
  return wasm._export_shape(inputShape.selectedIndex, [inputScaleX.value, inputScaleY.value, inputScaleZ.value], args);
}

function download(filename, text) {
  let element = document.createElement('a');
  element.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(text));
  element.setAttribute('download', filename);

  element.style.display = 'none';
  document.body.appendChild(element);

  element.click();

  document.body.removeChild(element);
}

// ==== ANIMATION LOOP
const animate = function() {
  webClient.drawSceneIf();

  requestAnimationFrame(animate);
}
requestAnimationFrame(animate);

generate();

// ==== NAVBAR
const navbarShapes = document.getElementById('navbar-shapes');
const navbarAbout = document.getElementById('navbar-about');
const navbarEditor = document.getElementById('navbar-editor');
const navbarHowToUse = document.getElementById('navbar-how-to-use');

const sectionShapes = document.getElementById('shapes');
const sectionAbout = document.getElementById('about');
const sectionEditor = document.getElementById('editor');
const sectionHowToUse = document.getElementById('how-to-use');

navbarShapes.onclick = function () {
  sectionShapes.scrollIntoView(true);
}

navbarAbout.onclick = function () {
  sectionAbout.scrollIntoView(true);
}

navbarEditor.onclick = function () {
  sectionEditor.scrollIntoView(true);
}

navbarHowToUse.onclick = function () {
  sectionHowToUse.scrollIntoView(true);
}

// ==== SHAPES
const shapesPlane = document.getElementById('shapes-plane');
const shapesDisk = document.getElementById('shapes-disk');
const shapesCube = document.getElementById('shapes-cube');
const shapesSphere = document.getElementById('shapes-sphere');
const shapesCylinder = document.getElementById('shapes-cylinder');
const shapesTube = document.getElementById('shapes-tube');

shapesPlane.onclick = () => shapeOnClick(0);
shapesDisk.onclick = () => shapeOnClick(1);
shapesCube.onclick = () => shapeOnClick(2);
shapesSphere.onclick = () => shapeOnClick(3);
shapesCylinder.onclick = () => shapeOnClick(4);
shapesTube.onclick = () => shapeOnClick(5);

function shapeOnClick(index) {
  inputShape.selectedIndex = index;
  generate();
  updateConfigurations();
  sectionEditor.scrollIntoView(true);
}

// ==== RESIZE
const canvas = document.getElementById('canvas');
const canvasParent = document.getElementById('canvasParent');
const gl = canvas.getContext("webgl");

// If the viewport of the website changes, we also want to reset te resolution of the canvas acordingly. Then we want to call drawscene in the wasmmodule again to ensure that the viewport is set correctly.
function resize() {
  let displayWidth = canvasParent.offsetWidth;
  let displayHeight = canvasParent.offsetHeight;

  gl.canvas.width = displayWidth;
  gl.canvas.height = displayHeight;

  webClient.drawScene();

  console.log('==== RESIZE ====');
}

// Add the resize function as event listener and call it on startup because sometimes the events are called before we add the listener.
window.onresize = resize;
window.onload = resize;
resize();

// ==== ONCHANGE
// This section ensures that the right input elements are displayed for each shape.

const itemSubdivisions = document.getElementsByClassName('subdivisions');
const itemRadius = document.getElementsByClassName('radius');
const itemSides = document.getElementsByClassName('sides');
const itemInnerRadius = document.getElementsByClassName('innerRadius');
const itemOuterRadius = document.getElementsByClassName('outerRadius');

inputShape.addEventListener('change', updateConfigurations);

updateConfigurations();

function updateConfigurations() {
  console.log('on change');

  switch(inputShape.value) {
    case 'plane':
      setDisplay(itemSubdivisions, true);
      setDisplay(itemRadius, false);
      setDisplay(itemSides, false);
      setDisplay(itemInnerRadius, false);
      setDisplay(itemOuterRadius, false);
      break;
    case 'disk':
      setDisplay(itemSubdivisions, false);
      setDisplay(itemRadius, true);
      setDisplay(itemSides, true);
      setDisplay(itemInnerRadius, false);
      setDisplay(itemOuterRadius, false);
      break;
    case 'cube':
      setDisplay(itemSubdivisions, true);
      setDisplay(itemRadius, false);
      setDisplay(itemSides, false);
      setDisplay(itemInnerRadius, false);
      setDisplay(itemOuterRadius, false);
      break;
    case 'sphere':
      setDisplay(itemSubdivisions, true);
      setDisplay(itemRadius, false);
      setDisplay(itemSides, false);
      setDisplay(itemInnerRadius, false);
      setDisplay(itemOuterRadius, false);
      break;
    case 'cylinder':
      setDisplay(itemSubdivisions, false);
      setDisplay(itemRadius, true);
      setDisplay(itemSides, true);
      setDisplay(itemInnerRadius, false);
      setDisplay(itemOuterRadius, false);
      break;
    case 'tube':
      setDisplay(itemSubdivisions, false);
      setDisplay(itemRadius, false);
      setDisplay(itemSides, true);
      setDisplay(itemInnerRadius, true);
      setDisplay(itemOuterRadius, true);
      break;
  }

  generate();
}

// Set the visibility of all the elements in an array. Indicate with show if it is visible or not.
function setDisplay(elements, show) {
  for (let i = 0; i < elements.length; i++) {
    if (show) {
      elements[i].style.display = "block";
    }
    else {
      elements[i].style.display = "none";
    }
  }
}