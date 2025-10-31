# Benchmarks

Performance benchmarks for the Jounce compiler and related systems.

## 📊 Available Benchmarks

### compiler_bench.rs
**Full compilation pipeline benchmarking**
- End-to-end compilation performance
- Measures lexing, parsing, analysis, and codegen
- Includes WASM optimization benchmarks
- Usage: `cargo bench --bench compiler_bench`

### cache_performance.rs
**Compilation cache system benchmarking**
- Cache hit/miss performance
- File system I/O benchmarks
- Cache invalidation timing
- Usage: `cargo bench --bench cache_performance`

### utility_generation.rs
**CSS utility generation benchmarking**
- CSS utility class generation speed
- 457 classes generation performance
- Output size metrics
- Usage: `cargo bench --bench utility_generation`

## 🚀 Running Benchmarks

**Run all benchmarks:**
```bash
cargo bench
```

**Run specific benchmark:**
```bash
cargo bench --bench compiler_bench
cargo bench --bench cache_performance
cargo bench --bench utility_generation
```

**Run with output:**
```bash
cargo bench -- --nocapture
```

## 📈 Benchmark Results

Benchmarks use [Criterion.rs](https://github.com/bheisler/criterion.rs) for statistical analysis.

Results are stored in `target/criterion/` directory:
- HTML reports for visualization
- Statistical analysis
- Comparison with baseline

## 🎯 Performance Goals

**Compiler:**
- < 100ms for small apps (< 500 lines)
- < 1s for medium apps (< 5000 lines)
- Linear scaling with code size

**Cache:**
- < 5ms for cache hits
- < 50ms for cache misses
- 90%+ hit rate in development

**Utility Generation:**
- < 10ms for full 457 class generation
- < 1ms for incremental updates

## 📝 Adding New Benchmarks

Create a new file in `benches/`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn my_benchmark(c: &mut Criterion) {
    c.bench_function("my_feature", |b| {
        b.iter(|| {
            // Code to benchmark
            black_box(my_function());
        });
    });
}

criterion_group!(benches, my_benchmark);
criterion_main!(benches);
```

Then add to `Cargo.toml`:
```toml
[[bench]]
name = "my_benchmark"
harness = false
```

## 🔗 Resources

- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Benchmarking Best Practices](https://doc.rust-lang.org/1.7.0/book/benchmark-tests.html)
