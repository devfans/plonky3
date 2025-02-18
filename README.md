# plonky3


## Poseidon2 benchmark
```bash

RUSTFLAGS="-Ctarget-cpu=native" cargo run --example prove_poseidon2 --release --features parallel 

```


INFO     generate vectorized Poseidon2 trace [ 200ms | 100.00% ]
INFO     prove [ 1.20s | 1.44% / 100.00% ]
INFO     ┝━ commit to trace data [ 647ms | 18.30% / 53.77% ]
INFO     │  ┕━ coset_lde_batch [ 427ms | 35.46% ] dims: 1320x131072 | added_bits: 1
INFO     ┝━ compute quotient polynomial [ 326ms | 27.08% ]
INFO     ┝━ commit to quotient poly chunks [ 24.0ms | 0.94% / 1.99% ]
INFO     │  ┝━ coset_lde_batch [ 6.63ms | 0.55% ] dims: 4x131072 | added_bits: 1
INFO     │  ┕━ coset_lde_batch [ 6.12ms | 0.51% ] dims: 4x131072 | added_bits: 1
INFO     ┕━ open [ 189ms | 0.02% / 15.72% ]
INFO        ┝━ compute_inverse_denominators [ 7.93ms | 0.66% ]
INFO        ┝━ evaluate matrix [ 30.7ms | 0.00% / 2.55% ] dims: 1320x262144
INFO        │  ┕━ compute opened values with Lagrange interpolation [ 30.6ms | 2.55% ]
INFO        ┝━ evaluate matrix [ 35.7ms | 0.00% / 2.97% ] dims: 1320x262144
INFO        │  ┕━ compute opened values with Lagrange interpolation [ 35.7ms | 2.97% ]
INFO        ┝━ evaluate matrix [ 1.65ms | 0.00% / 0.14% ] dims: 4x262144
INFO        │  ┕━ compute opened values with Lagrange interpolation [ 1.64ms | 0.14% ]
INFO        ┝━ evaluate matrix [ 1.70ms | 0.00% / 0.14% ] dims: 4x262144
INFO        │  ┕━ compute opened values with Lagrange interpolation [ 1.69ms | 0.14% ]
INFO        ┝━ reduce matrix quotient [ 53.1ms | 0.05% / 4.42% ] dims: 1320x262144
INFO        │  ┝━ compress mat [ 47.1ms | 3.91% ]
INFO        │  ┝━ reduce rows [ 3.03ms | 0.25% ]
INFO        │  ┕━ reduce rows [ 2.40ms | 0.20% ]
INFO        ┝━ reduce matrix quotient [ 4.76ms | 0.01% / 0.40% ] dims: 4x262144
INFO        │  ┝━ compress mat [ 1.56ms | 0.13% ]
INFO        │  ┕━ reduce rows [ 3.13ms | 0.26% ]
INFO        ┝━ reduce matrix quotient [ 3.44ms | 0.00% / 0.29% ] dims: 4x262144
INFO        │  ┝━ compress mat [ 1.02ms | 0.09% ]
INFO        │  ┕━ reduce rows [ 2.38ms | 0.20% ]
INFO        ┕━ FRI prover [ 50.0ms | 0.04% / 4.15% ]
INFO           ┝━ commit phase [ 40.8ms | 3.39% / 3.39% ]
INFO           │  ┕━ divide_by_height [ 18.4µs | 0.00% ] dims: 1x2
INFO           ┝━ grind for proof-of-work witness [ 7.14ms | 0.59% ]
INFO           ┕━ query phase [ 1.50ms | 0.12% ]
Proof size: 1227084 bytes
INFO     verify [ 86.2ms | 97.34% / 100.00% ]
INFO     ┕━ infer log of constraint degree [ 2.29ms | 2.66% ]
Proof Verified Successfully
