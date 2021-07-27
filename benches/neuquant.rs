#![feature(test)]

extern crate color_quant;
extern crate test;

use color_quant::NeuQuant;
use test::{black_box, Bencher};

fn make_pixels() -> Vec<u8> {
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
    pixels
}

#[bench]
fn bench_map_pixel(b: &mut Bencher) {
    let pixels = make_pixels();
    let nq = NeuQuant::new(10, 256, &pixels);
    let mut i = 0u8;
    b.iter(|| {
        i = i.wrapping_add(1);
        let mut color = black_box([123, i, 4, 255]);
        nq.map_pixel(&mut color[..]);
    })
}

#[bench]
fn bench_new_nq(b: &mut Bencher) {
    let pixels = make_pixels();
    b.iter(|| NeuQuant::new(10, 128, &pixels));
}
