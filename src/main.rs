use zero_knowledge::{prover::Prover, verifier::Verifier};

fn main() {
    const PRIME: u32 = 0xFFFFFFFF;
    let prover = Prover::new(PRIME);
    let verifier = Verifier::new(PRIME);

    let witness = [3, 4, 21];
    let proof = prover.generate_proof(&witness);
    let is_valid = verifier.verify(&proof);
    
    println!("Proof is valid: {}", is_valid);
}
