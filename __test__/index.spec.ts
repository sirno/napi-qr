import test from "ava";

import { qr, qrSvg, Shape, SvgOptions } from "../index";

test("test bitmap qr", (t) => {
  const target = "Hello QR Coded Planet";
  qr(target);
  t.true(true, "pass");
});

test("test square qr", (t) => {
  const target = "Hello QR Coded Planet";
  const options = new SvgOptions();
  qrSvg(target, options);
  t.true(true, "pass");
});

test("test diamond qr", (t) => {
  const target = "Hello QR Coded Planet";
  const options = new SvgOptions();
  options.shape = Shape.Diamond;
  options.margin = 5;
  qrSvg(target, options);
  t.true(true, "pass");
});

test("test circle qr", (t) => {
  const target = "Hello QR Coded Planet";
  const options = new SvgOptions();
  options.shape = Shape.Circle;
  qrSvg(target, options);
  t.true(true, "pass");
});

test("test rounded square qr", (t) => {
  const target = "Hello QR Coded Planet";
  const options = new SvgOptions();
  options.shape = Shape.RoundedSquare;
  const svg = qrSvg(target, options);
  t.log(svg);
  t.true(true, "pass");
});
