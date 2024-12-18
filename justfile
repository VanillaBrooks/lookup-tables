bench_store NAME:
    cargo export target/benchmarks -- bench --bench={{NAME}}

bench_compare NAME:
    cargo bench -q --bench={{NAME}} -- compare target/benchmarks/bench_table1d

bench NAME:
    cargo bench -q --bench={{NAME}} -- compare

