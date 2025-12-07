# Boucle for avec range

**Level:** ⭐ BEGINNER

**Objective:** Itérer sur une plage de nombres avec for

**Problem:**

Utilisez for i in 1..6 { } pour afficher 1 à 5. Comparez avec 1..=6.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- for est l'idiom Rust pour les itérations ; c'est plus sûr que while.
- 1..6 est une plage exclusive (1-5) ; 1..=6 est inclusive (1-6).
