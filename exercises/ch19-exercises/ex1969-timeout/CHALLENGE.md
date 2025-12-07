# timeout

**Level:** ⭐⭐⭐ ADVANCED

**Objective:** Ajouter un timeout à une future

**Problem:**

Utilisez `tokio::time::timeout()` pour limiter la durée.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Prévention des futures bloquantes infinies.
- timeout retourne Err(Elapsed) si la future dépasse le délai.
