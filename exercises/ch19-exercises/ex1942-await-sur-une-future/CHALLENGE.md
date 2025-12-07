# await sur une Future

**Level:** ⭐⭐ INTERMEDIATE

**Objective:** Attendre une Future avec await

**Problem:**

Appelez `my_future.await` pour attendre le résultat.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- await ne peut être utilisé que dans async.
- .await suspend et reprend à ce point.
