cargo clippy ${workspace} --all-targets \
  --features "${features}" -- -D warnings -D future-incompatible \
  -D nonstandard-style -D rust-2021-compatibility -D rust-2024-compatibility \
   -D unused -D clippy::all  -D clippy::unwrap_used -D clippy::unwrap_in_result