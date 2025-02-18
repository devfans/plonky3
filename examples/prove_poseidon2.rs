use clap::Parser;
use p3_baby_bear::{BabyBear, GenericPoseidon2LinearLayersBabyBear, Poseidon2BabyBear};
use p3_dft::Radix2DitParallel;
use p3_field::Field;
use p3_commit::ExtensionMmcs;
use p3_examples::airs::ProofObjective;
use p3_challenger::{DuplexChallenger, SerializingChallenger32, HashChallenger};
use p3_examples::dfts::DftChoice;
use p3_examples::parsers::{DftOptions, FieldOptions, MerkleHashOptions, ProofOptions};
use p3_examples::proofs::{
    prove_m31_keccak, prove_m31_poseidon2, prove_monty31_keccak, prove_monty31_poseidon2,
    report_result,
};
use p3_fri::{FriConfig, TwoAdicFriPcs};
use p3_field::extension::BinomialExtensionField;
use p3_keccak_air::KeccakAir;
use p3_koala_bear::{GenericPoseidon2LinearLayersKoalaBear, KoalaBear, Poseidon2KoalaBear};
use p3_mersenne_31::{GenericPoseidon2LinearLayersMersenne31, Mersenne31, Poseidon2Mersenne31};
use p3_monty_31::dft::RecursiveDft;
use p3_poseidon2_air::{RoundConstants, VectorizedPoseidon2Air};
use p3_uni_stark::{prove, verify, Proof, StarkConfig, StarkGenericConfig};
use p3_merkle_tree::MerkleTreeMmcs;
use p3_symmetric::{
    CompressionFunctionFromHasher, PaddingFreeSponge, SerializingHasher32To64, TruncatedPermutation,
};
use p3_keccak::{Keccak256Hash, KeccakF};

use rand::rng;
use tracing_forest::util::LevelFilter;
use tracing_forest::ForestLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

// General constants for constructing the Poseidon2 AIR.
const P2_WIDTH: usize = 16;
const P2_HALF_FULL_ROUNDS: usize = 4;
const P2_LOG_VECTOR_LEN: u8 = 3;
const P2_VECTOR_LEN: usize = 1 << P2_LOG_VECTOR_LEN;


fn main() {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    Registry::default()
        .with(env_filter)
        .with(ForestLayer::default())
        .init();

    let proof_goal = {
        let constants: RoundConstants<p3_monty_31::MontyField31<p3_koala_bear::KoalaBearParameters>, 16, 4, 20> = RoundConstants::from_rng(&mut rng());

        // Field specific constants for constructing the Poseidon2 AIR.
        const SBOX_DEGREE: u64 = 3;
        const SBOX_REGISTERS: usize = 0;
        const PARTIAL_ROUNDS: usize = 20;

        let p2_air: VectorizedPoseidon2Air<
            KoalaBear,
            GenericPoseidon2LinearLayersKoalaBear,
            P2_WIDTH,
            SBOX_DEGREE,
            SBOX_REGISTERS,
            P2_HALF_FULL_ROUNDS,
            PARTIAL_ROUNDS,
            P2_VECTOR_LEN,
        > = VectorizedPoseidon2Air::new(constants);
        ProofObjective::Poseidon2(p2_air)
    };
    let dft = DftChoice::Parallel(Radix2DitParallel::default());
    let perm16 = Poseidon2KoalaBear::<16>::new_from_rng_128(&mut rng());
    let perm24 = Poseidon2KoalaBear::<24>::new_from_rng_128(&mut rng());

    
    type EF = BinomialExtensionField<KoalaBear, 4>;
    type Poseidon2Sponge<Perm24> = PaddingFreeSponge<Perm24, 24, 16, 8>;
    type Poseidon2Compression<Perm16> = TruncatedPermutation<Perm16, 2, 8, 16>;
    type Poseidon2MerkleMmcs<F, Perm16, Perm24> = MerkleTreeMmcs<
        <F as Field>::Packing,
        <F as Field>::Packing,
        Poseidon2Sponge<Perm24>,
        Poseidon2Compression<Perm16>,
        8,
    >;
    const KECCAK_VECTOR_LEN: usize = p3_keccak::VECTOR_LEN;

    type KeccakCompressionFunction =
    CompressionFunctionFromHasher<PaddingFreeSponge<KeccakF, 25, 17, 4>, 2, 4>;
    type KeccakMerkleMmcs<F> = MerkleTreeMmcs<
        [F; KECCAK_VECTOR_LEN],
        [u64; KECCAK_VECTOR_LEN],
        SerializingHasher32To64<PaddingFreeSponge<KeccakF, 25, 17, 4>>,
        KeccakCompressionFunction,
        4,
    >;
    type KeccakStarkConfig<F, EF, DFT> = StarkConfig<
        TwoAdicFriPcs<F, DFT, KeccakMerkleMmcs<F>, ExtensionMmcs<F, EF, KeccakMerkleMmcs<F>>>,
        EF,
        SerializingChallenger32<F, HashChallenger<u8, Keccak256Hash, 32>>,
    >;

    let val_mmcs = {
        let u64_hash = PaddingFreeSponge::<KeccakF, 25, 17, 4>::new(KeccakF {});
        let field_hash = SerializingHasher32To64::new(u64_hash);
        let compress = KeccakCompressionFunction::new(u64_hash);
        KeccakMerkleMmcs::new(field_hash, compress)
    };
    let challenge_mmcs = ExtensionMmcs::<_, EF, _>::new(val_mmcs.clone());
    let fri_config = FriConfig {
        log_blowup: 1,
        log_final_poly_len: 0,
        num_queries: 100,
        proof_of_work_bits: 16,
        mmcs: challenge_mmcs,
    };

    let num_hashes = 1 << 20;
    let trace = match proof_goal {
        ProofObjective::Poseidon2(ref p2_air) => {
            p2_air.generate_vectorized_trace_rows(num_hashes, fri_config.log_blowup)
        },
        _ => panic!("invalid hash objective")
    };
    let pcs = TwoAdicFriPcs::new(dft, val_mmcs, fri_config);

    let config = KeccakStarkConfig::new(pcs);

    let mut proof_challenger = SerializingChallenger32::from_hasher(vec![], Keccak256Hash {});
    let mut verif_challenger = SerializingChallenger32::from_hasher(vec![], Keccak256Hash {});

    let proof = prove(&config, &proof_goal, &mut proof_challenger, trace, &vec![]);
    {
        let proof_bytes = bincode::serialize(&proof).expect("Failed to serialize proof");
        println!("Proof size: {} bytes", proof_bytes.len());
        std::fs::write(
            "proof_poseidon2.json",
            serde_json::to_string(&proof).unwrap(),
        )
        .unwrap();
    }

    let result = verify(&config, &proof_goal, &mut verif_challenger, &proof, &vec![]);
    if let Err(e) = result {
        panic!("{:?}", e);
    } else {
        println!("Proof Verified Successfully")
    }
}