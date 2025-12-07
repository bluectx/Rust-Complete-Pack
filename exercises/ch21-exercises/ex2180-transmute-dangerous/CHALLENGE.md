# Transmute (dangerous!)

**Level:** ⭐⭐⭐ ADVANCED

**Objective:** Comprendre les risques de transmute

**Problem:**

Utilisez std::mem::transmute avec extrême prudence (éducationnel uniquement).

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Extrêmement dangereux ; évitez sauf si vous savez exactement ce que vous faites.
- transmute réinterprète les bits bruts.
