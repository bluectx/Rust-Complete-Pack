# select! pour l'une des premières

**Level:** ⭐⭐⭐ ADVANCED

**Objective:** Utiliser tokio::select! pour la première future prête

**Problem:**

Lancez plusieurs tâches et utilisez select! pour la première.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Utile pour les timeouts et les cancellations.
- select! retourne quand une future est ready.
