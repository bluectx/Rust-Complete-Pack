# Shadowing de variables

**Level:** ⭐ BEGINNER

**Objective:** Comprendre le shadowing (réassignation de nom)

**Problem:**

Créez une variable `x` entière, puis redéclarez `x` en tant que String avec le même nom. Affichez les deux.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Shadowing est différent de mutation ; c'est une redéclaration, pas une modification.
- Le shadowing permet de réutiliser un nom de variable avec un type ou une valeur différente.
