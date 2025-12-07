# Black box benchmarking

**Level:** ⭐ BEGINNER

**Objective:** Écrire des benchmarks qui ne sont pas optimisés à mort

**Problem:**

Utilisez test::black_box() pour empêcher l'optimisation.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Benchmarks réalistes.
- black_box empêche le compilateur d'optimiser le benchmark.
