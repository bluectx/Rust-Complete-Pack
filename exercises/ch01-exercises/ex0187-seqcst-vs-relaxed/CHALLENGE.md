# SeqCst vs Relaxed

**Level:** ⭐ BEGINNER

**Objective:** Comprendre les compromis entre consistency et performance

**Problem:**

Comparez Ordering::SeqCst (sûr mais lent) vs Relaxed (rapide mais subtil).

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Trade-off : sécurité vs performance.
- SeqCst = guarantee ; Relaxed = aucune.
