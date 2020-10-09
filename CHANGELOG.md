## 1.1.0

- replace the `clamp!` macro with the `clamp` function (https://github.com/image-rs/color_quant/pull/8)
- Unify with `image::math::nq` as per https://github.com/image-rs/image/issues/1338 (https://github.com/image-rs/color_quant/pull/10)
  - A new method `lookup` from image::math::nq is added
  - more references in docs
  - some syntax improvements and better names for functions borrowed from  `image::math::nq`