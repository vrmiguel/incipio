use std::{fs::File, io::Write};

fn main() {
    let seed: [u8; 512] = std::array::from_fn(|_| fastrand::u8(..));

    let mut rand_file = File::create("src/rand_seed.rs")
        .expect("Failed to create rand_seed.rs");
    write!(
        rand_file,
        "pub const SEED: &[u8; 512] = &{:?};",
        seed
    )
    .expect("Failed to write to rand_seed.rs");
}
