# Channel pour inter-thread communication

**Level:** ⭐⭐ INTERMEDIATE

**Objective:** Communiquer entre threads avec channels

**Problem:**

Utilisez `mpsc::channel()` pour envoyer des messages.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Alternative aux locks pour communiquer.
- mpsc = multi-producer, single-consumer.
