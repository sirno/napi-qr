import test from "ava";

import { qr, qrPng, qrSvg, Shape, SvgOptions, ModuleStyle } from "../index";
import fs from "fs";

function writeTestFile(data: string) {
  fs.writeFileSync("test.svg", data, { flush: true });
}

function pngToDataUri(filePath: string) {
  try {
    // Read the file as a buffer
    const fileBuffer = fs.readFileSync(filePath);

    // Convert the buffer to base64
    const base64Data = fileBuffer.toString("base64");

    // Create the data URI
    const dataUri = `data:image/png;base64,${base64Data}`;

    return dataUri;
  } catch (error) {
    console.error("Error converting PNG to data URI:", error);
    throw error;
  }
}

test("test bitmap qr", (t) => {
  const target = "Hello QR Coded Planet";
  qr(target);
  t.true(true, "pass");
});

test("test default qr", (t) => {
  const target = "Hello QR Coded Planet";
  const options = new SvgOptions();
  const svg = qrSvg(target, options);
  t.true(true, "pass");
});

test("test diamond qr", (t) => {
  const target = "Hello QR Coded Planet";
  const module_style = new ModuleStyle(Shape.Diamond, 1);
  const options = new SvgOptions();
  options.style = module_style;
  options.margin = 5;
  const svg = qrSvg(target, options);
  // writeTestFile(svg);
  t.true(true, "pass");
});

test("test circle qr", (t) => {
  const target = "Hello QR Coded Planet";
  const module_style = new ModuleStyle(Shape.Circle, 0.8);
  const options = new SvgOptions();
  options.style = module_style;
  const svg = qrSvg(target, options);
  // writeTestFile(svg);
  t.true(true, "pass");
});

test("test rounded square qr", (t) => {
  const target = "Hello QR Coded Planet";
  const module_style = new ModuleStyle(Shape.RoundedSquare, 1);
  const options = new SvgOptions();
  options.style = module_style;
  const svg = qrSvg(target, options);
  // writeTestFile(svg);
  t.true(true, "pass");
});

test("test connected qr", (t) => {
  const target = "Hello QR Coded Planet";
  const module_style = new ModuleStyle(Shape.Connected, 1);
  const options = new SvgOptions();
  options.style = module_style;
  // const image_uri = pngToDataUri("./__test__/pyramids.png");
  // options.image = image_uri;
  const svg = qrSvg(target, options);
  writeTestFile(svg);
  // t.log(svg);
  t.true(true, "pass");
});

test("test default qr image", (t) => {
  const target = "Hello QR Coded Planet";
  const options = new SvgOptions();
  qrPng(target, options);
  t.true(true, "pass");
});
