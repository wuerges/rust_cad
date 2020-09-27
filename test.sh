echo testing for "$@"
cargo run --release "$@"
../iccad2017b_cxx/evaluator/eval_3 "$@"