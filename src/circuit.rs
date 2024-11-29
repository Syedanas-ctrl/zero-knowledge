/**
 * Represents a simple computational circuit
 * using a quadratic equation constraint
 */
pub struct Circuit {}

impl Circuit {
    pub fn new() -> Self {
        Circuit {}
    }

    /**
     * Verify if the witness satisfies the circuit constraint
     * Example constraint: x^2 + xy = z
     * @param witness Computation inputs [x, y, z]
     * @returns Boolean indicating circuit satisfaction
     */
    pub fn verify(&self, witness: &[u32]) -> bool {
        if witness.len() != 3 {
            return false
        }
        let x = witness[0];
        let y = witness[1];
        let z = witness[2];

        return (x * x + x*y) == z;
    }
}
