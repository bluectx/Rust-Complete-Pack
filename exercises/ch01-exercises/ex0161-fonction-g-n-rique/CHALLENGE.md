# Fonction générique

**Level:** ⭐ BEGINNER

**Objective:** Écrire une fonction qui fonctionne avec plusieurs types

**Problem:**

Écrivez `fn maximum<T: PartialOrd>(a: T, b: T) -> T` (même si elle ne fonctionne qu'avec Copy).

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Les génériques permettent la réutilisabilité tout en conservant la sécurité de type.
- `<T>` introduit un paramètre de type générique.
