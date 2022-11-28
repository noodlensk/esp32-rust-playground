convert rust.png -depth 1 gray:rust.raw
const raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("../data/images/rust.raw"), 64);
