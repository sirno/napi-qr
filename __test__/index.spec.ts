import test from "ava";

import { qr, qrSvg, Shape, SvgOptions, ModuleStyle } from "../index";

test("test bitmap qr", (t) => {
  const target = "Hello QR Coded Planet";
  qr(target);
  t.true(true, "pass");
});

test("test default qr", (t) => {
  const target = "Hello QR Coded Planet";
  const options = new SvgOptions();
  qrSvg(target, options);
  t.true(true, "pass");
});

test("test diamond qr", (t) => {
  const target = "Hello QR Coded Planet";
  const module_style = new ModuleStyle(Shape.Diamond, 1);
  const options = new SvgOptions();
  options.style = module_style;
  options.margin = 5;
  qrSvg(target, options);
  t.true(true, "pass");
});

test("test circle qr", (t) => {
  const target = "Hello QR Coded Planet";
  const module_style = new ModuleStyle(Shape.Circle, 0.8);
  const options = new SvgOptions();
  options.style = module_style;
  const svg = qrSvg(target, options);
  t.log(svg);
  t.true(true, "pass");
});

test("test rounded square qr", (t) => {
  const target = "Hello QR Coded Planet";
  const module_style = new ModuleStyle(Shape.RoundedSquare, 1);
  const options = new SvgOptions();
  options.style = module_style;
  qrSvg(target, options);
  t.true(true, "pass");
});
