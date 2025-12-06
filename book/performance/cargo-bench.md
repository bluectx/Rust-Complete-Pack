# cargo bench - Mesurer les Performances! ⚡

## Learning Objectives

- Utiliser cargo bench pour mesurer les performances (c'est facile!)
- Comprendre les benchmarks Rust
- Interpréter les résultats de benchmark
- Optimiser le code basé sur les benchmarks

## Core Explanation

### For Absolute Beginners - C'est Comme un Chronomètre! ⏱️

Imaginez un **chronomètre** ⏱️ pour mesurer une course:
- **cargo bench** = Le chronomètre qui mesure votre code
- Il vous dit combien de temps prend chaque fonction
- Vous pouvez comparer différentes versions!

C'est **exactement** comme cargo bench fonctionne! C'est **super pratique**!

## Schéma Visuel - Benchmark

```
┌─────────────────────────────────────────┐
│  ⏱️ BENCHMARK = CHRONOMÈTRE ⏱️        │
├─────────────────────────────────────────┤
│                                         │
│  Fonction à tester                      │
│         │                                │
│         ▼ cargo bench                    │
│  ⏱️ Mesure le temps                     │
│         │                                │
│         ▼ Résultat                       │
│  "fib 20: 123.45 ns"                    │
│                                         │
│  Comparez différentes versions! ✅     │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Chronomètre" - cargo bench mesure le temps d'exécution de votre code comme un chronomètre mesure une course!

## Code Examples

### Example 1: Benchmark Basique

```rust
// Dans Cargo.toml, ajouter:
// [dev-dependencies]
// criterion = { version = "1.0", features = ["html_reports"] }

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn bench_fib(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, bench_fib);
criterion_main!(benches);
```

**Exécution:**

```bash
cargo bench
```

**Output:**
```
fib 20                  time:   [123.45 ns 124.56 ns 125.67 ns]
```

## Interpréter les Résultats

- **time** = Temps d'exécution moyen
- **ns** = Nanosecondes (très précis!)
- Comparer pour voir quelle version est plus rapide
- Les variations sont normales (mesure statistique)

## Best Practices

1. **Utiliser criterion** : Plus précis que les benchmarks simples
2. **Tester plusieurs fois** : Les résultats varient
3. **Comparer** : Tester différentes implémentations
4. **Optimiser** : Utiliser les résultats pour améliorer

## Official Resources

- [@official Criterion.rs](https://github.com/bheisler/criterion.rs)
- [Cargo Book - Benchmarks](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#benchmarks)

