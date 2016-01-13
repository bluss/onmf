extern crate onmf;
use onmf::helpers::{ToImage};
use onmf::testimage_generator;

extern crate nalgebra;
use nalgebra::{DMat};

extern crate rand;
use rand::{StdRng, SeedableRng};

fn main() {
    for (i, factor) in testimage_generator::static_factors::<f64>().enumerate() {
        factor.save_to_png(&format!("factor-{}.png", i)[..]).unwrap();
    }

    for (i, factor) in testimage_generator::horizontal_evolving_factors::<f64>().enumerate() {
        factor.save_to_png(&format!("horizontal-{}.png", i)[..]).unwrap();
    }

    for (i, factor) in testimage_generator::vertical_evolving_factors::<f64>().enumerate() {
        factor.save_to_png(&format!("vertical-{}.png", i)[..]).unwrap();
    }

    // let seed: &[_] = &[1, 2, 3, 4];
    // let mut rng: StdRng = SeedableRng::from_seed(seed);

    // let testimages: Box<Iterator<Item=Dmat<f64>>> =
    //     Testimage::testimages(1, &mut rng);
    // for (step, i, factor) in testimages.enumerate() {
    //     factor.save_luma01_to_png(&format!("test-{}-{}.png", step, i)[..]).unwrap();
    // }
}
