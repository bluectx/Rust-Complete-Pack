# async fn basique

**Level:** ⭐⭐ INTERMEDIATE

**Objective:** Écrire une fonction asynchrone

**Problem:**

Écrivez `async fn my_func() { }` et comprenez qu'elle retourne une Future.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- async n'exécute rien jusqu'à .await.
- async fn crée une Future qui peut être awaited.
