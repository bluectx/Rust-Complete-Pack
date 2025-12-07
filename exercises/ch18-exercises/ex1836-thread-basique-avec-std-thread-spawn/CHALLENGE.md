# Thread basique avec std::thread::spawn

**Level:** ⭐⭐ INTERMEDIATE

**Objective:** Créer un thread et attendre son exécution

**Problem:**

Utilisez `std::thread::spawn(|| { })` et `.join()`.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Les threads Rust sont des 1:1 OS threads.
- spawn crée un nouveau thread ; join attend son terme.
