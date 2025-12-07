# Trait bound en paramètre

**Level:** ⭐ BEGINNER

**Objective:** Utiliser un trait comme contrainte générique

**Problem:**

Écrivez une fonction `fn faire_parler(animal: &impl Animal)` qui appelle faire_bruit.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Cela permet du polymorphisme ad-hoc.
- `&impl TraitName` = le paramètre doit implémenter TraitName.
