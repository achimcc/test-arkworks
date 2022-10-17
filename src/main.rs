use ark_bls12_381::{Fr as ScalarField, G1Projective as G};
use ark_ec::{scalar_mul::variable_base::VariableBaseMSM, ScalarMul};
use ark_ff::UniformRand;

fn main() {
    const SAMPLES: usize = 1 << 10;

    let mut rng = ark_std::test_rng();

    let v = (0..SAMPLES)
        .map(|_| ScalarField::rand(&mut rng))
        .collect::<Vec<_>>();
    let g = (0..SAMPLES)
        .map(|_| G::rand(&mut rng))
        .map(|v| v)
        .collect::<Vec<_>>();
    let g = G::batch_convert_to_mul_base(&g);

    let result = G::msm(g.as_slice(), v.as_slice());
    println!("{:?}", result);
}
