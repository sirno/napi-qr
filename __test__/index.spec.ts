import test from 'ava'

import { qr, qrSvg, Shape, SvgOptions } from '../index'

test('test bitmap qr', (t) => {
  const target = 'Hello QR Coded Planet'
  t.log(qr(target))
  t.true(true, 'pass')
})

test('test square qr', (t) => {
  const target = 'Hello QR Coded Planet'
  t.log(qrSvg(target, new SvgOptions()))
  t.true(true, 'pass')
})

test('test diamond qr', (t) => {
  const target = 'Hello QR Coded Planet'
  const options = new SvgOptions()
  options.shape = Shape.Diamond
  t.log(qrSvg(target, new SvgOptions()))
  t.true(true, 'pass')
})

test('test circle qr', (t) => {
  const target = 'Hello QR Coded Planet'
  const options = new SvgOptions()
  options.shape = Shape.Circle
  t.log(qrSvg(target, new SvgOptions()))
  t.true(true, 'pass')
})
