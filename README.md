# matrix-multiply
Multi-threaded matrix multiplication in rust.

Goal of this project is to test the difference in performance of multithreaded and singlethreaded applications.

## Benchmark

| Number of threads | Matrix dimensions | execution time |
|-------------------|-------------------|----------------|
| 1                 | 1400              | 13s            |
| 4                 | 1400              | 3s             |

> Note: execution time may vary, depending on hardware

## How to use
 - Run `python3 gen.py 1000 > input.txt` to generate two random 1000x1000 matrices.
 - Run `cargo run --release < input.txt` for multithreaded matrix multiplication.
 - Run `cargo run --release -- -s < input.txt` for single matrix multiplication.
