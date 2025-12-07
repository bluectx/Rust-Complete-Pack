# Custom futures

**Level:** ⭐ BEGINNER

**Objective:** Implémenter le trait Future manuellement

**Problem:**

Créez une struct qui impl Future avec poll().

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Comprenez le cœur des futures.
- Future::poll() retourne Poll<Output> ou Pending.
