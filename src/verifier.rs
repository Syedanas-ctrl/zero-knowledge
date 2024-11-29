use crate::{field::Field, prover::ZKProof};

pub struct Verifier {
    field: Field,
}

impl Verifier {
    pub fn new(prime: u32) -> Self {
        Verifier {
            field: Field::new(prime),
        }
    }

    pub fn verify(&self, proof: &ZKProof) -> bool {
        // Verification checks
        // 1. Verify computational constraints
        let constraint_check = self.field.add(proof.a, proof.b) == proof.c;
        // 2. Check blinding factor doesn't reveal too much information
        let blinding_check = proof.random_blinding < 2000;

        constraint_check && blinding_check
    }
}
