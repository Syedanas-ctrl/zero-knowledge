use crate::{circuit::Circuit, field::Field};
use rand::Rng;

pub struct Prover {
    field: Field,
    circuit: Circuit
}
#[derive(Debug)]
pub struct ZKProof {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub random_blinding: u32,
}

impl Prover {
    pub fn new (prime: u32) -> Self {
        Prover {
            field: Field::new(prime),
            circuit: Circuit::new()
        }
    }

    pub fn generate_proof(&self, witness: &[u32]) -> ZKProof{
        if !self.circuit.verify(witness) {
            panic!("Invalid witness for the circuit");
        }

        let [x, y, z] = witness[..3] else {
            panic!("Witness must contain at least 3 values");
        };

        ZKProof {
            a: self.field.multiply(x, x),  // x^2
            b: self.field.multiply(x, y),  // xy
            c: z,                     // expected result
            random_blinding: self.generate_random_blinding()
          }
    }

    /**
   * Generate a random blinding factor to add zero-knowledge property
   * @returns Random bigint
   */
    fn generate_random_blinding(&self) -> u32 {
        rand::thread_rng().gen_range(0..1000) as u32
    }
}