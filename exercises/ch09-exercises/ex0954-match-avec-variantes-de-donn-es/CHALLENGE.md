# Match avec variantes de données

**Level:** ⭐ BEGINNER

**Objective:** Pattern match et extrait des données

**Problem:**

Créez un Message et extraisez ses données avec match.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- On peut capturer les données d'une variante dans le pattern.
- `match msg { Message::Text(s) => { // s est la String }, ... }`
