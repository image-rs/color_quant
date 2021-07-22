#![feature(array_chunks)]

extern crate color_quant;

use color_quant::NeuQuant;
use std::fs::write;

fn main() {
    let header = b"P6\n256\n256\n255\n";
    let mut pixels = vec![0; 4 * 256 * 256];
    let mut r: u8 = 0;
    let mut g: u8 = 0;
    let mut b: u8 = 0;

    for (i, p) in pixels.iter_mut().enumerate() {
        match i % 4 {
            0 => *p = r,
            1 => *p = g,
            2 => {
                *p = b;
                if let Some(next_r) = r.checked_add(1) {
                    r = next_r;
                    continue;
                }
                r = 0;
                if let Some(next_g) = g.checked_add(1) {
                    g = next_g;
                    continue;
                }
                g = 0;
                b += 1
            }
            3 => *p = 255,
            _ => unreachable!(),
        }
    }
    let raw = &header
        .into_iter()
        .chain(
            pixels
                .array_chunks::<4>()
                .map(|[r, g, b, _a]| [r, g, b])
                .flatten(),
        )
        .copied()
        .collect::<Vec<u8>>();
    write("img.ppm", raw).expect("Failed to write");
    let nq = NeuQuant::new(10, 256, &pixels);

    let quantized = &header
        .into_iter()
        .copied()
        .chain(
            pixels
                .array_chunks::<4>()
                .map(|&[r, g, b, a]| {
                    let mut color = [r, g, b, a];
                    nq.map_pixel(&mut color[..]);
                    let [r, g, b, _a] = color;
                    [r, g, b]
                })
                .flatten(),
        )
        .collect::<Vec<u8>>();
    write("quantized.ppm", quantized).expect("Failed to write");
}
