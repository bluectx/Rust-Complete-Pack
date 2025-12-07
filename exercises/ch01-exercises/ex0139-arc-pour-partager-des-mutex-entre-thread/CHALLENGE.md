# Arc pour partager des Mutex entre threads

**Level:** ⭐ BEGINNER

**Objective:** Utiliser Arc<Mutex<T>> pour ownership partagé synchronisé

**Problem:**

Créez `Arc<Mutex<T>>`, clonez, et modifiez depuis multiple threads.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Arc + Mutex = shared mutable state sûr.
- Arc = Rc thread-safe ; Mutex = synchronisation.
