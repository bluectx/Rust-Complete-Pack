# Exercise: Ownership - Partager une Burger! 🍔

**Level:** ⭐⭐ Intermediate

**Objective:** Comprendre et corriger un problème d'ownership. Vous allez apprendre à **partager une burger** correctement!

**Problem:**

Le code suivant ne compile pas car `s1` (votre burger 🍔) est déplacé vers `s2`. Corrigez-le en utilisant une des approches suivantes:
1. Utiliser seulement `s2` (Bob garde la burger)
2. Cloner `s1` (faire une deuxième burger)
3. Utiliser borrowing (prêter la burger sans la donner)

**Analogie:**

Imaginez que vous avez une **burger** 🍔:
- **Move** = Donner la burger à quelqu'un (vous ne l'avez plus!)
- **Clone** = Faire une copie de la burger (vous avez tous les deux une burger!)
- **Borrow** = Prêter la burger (vous l'avez toujours, mais quelqu'un la regarde!)

**Example Runs:**

```bash
$ cargo run
✅ Solution trouvée!
s2: hello
```

**Ce que vous allez créer:**

Une solution **élégante** au problème d'ownership! C'est **super satisfaisant** de comprendre comment partager correctement!

**Tests:**

Run: `cargo test`

All tests must pass.

**Tips:**

- Pensez à la burger! 🍔
- Essayez les 3 solutions pour voir laquelle vous préférez
- Le borrowing est souvent la meilleure solution!

