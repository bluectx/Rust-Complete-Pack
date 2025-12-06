# criterion.rs - Mesurer les Performances Facilement! ⚡

## Learning Objectives

- Utiliser criterion pour les benchmarks (c'est super facile!)
- Interpréter les résultats
- Comparer les performances
- Optimiser votre code comme un pro!

## Core Explanation

### For Absolute Beginners - C'est Comme Chronométrer une Course! ⏱️

Imaginez que vous voulez savoir quelle méthode est la plus rapide:
- **Criterion** = Un chronomètre super précis
- Il mesure combien de temps prend chaque méthode
- Vous pouvez comparer et choisir la plus rapide!

C'est **exactement** comme criterion fonctionne! C'est **super pratique**!

## Schéma Visuel - Criterion

```
┌─────────────────────────────────────────┐
│  ⚡ CRITERION = CHRONOMÈTRE ⚡          │
├─────────────────────────────────────────┤
│                                         │
│  Méthode 1: Calculer somme             │
│         │                                │
│         ▼ Criterion mesure              │
│  ⏱️ Temps: 5 ms                         │
│                                         │
│  Méthode 2: Calculer somme             │
│         │                                │
│         ▼ Criterion mesure              │
│  ⏱️ Temps: 3 ms                         │
│                                         │
│  ✅ Méthode 2 est plus rapide!          │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Chronomètre de Performance" - Criterion mesure précisément le temps d'exécution de votre code, comme un chronomètre de course, pour comparer et optimiser!

## Code Examples

### Example 1: Benchmark Basique (Super Facile!)

```rust
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

### Example 2: Comparer Deux Méthodes

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn method1(input: &[i32]) -> i32 {
    input.iter().sum()
}

fn method2(input: &[i32]) -> i32 {
    let mut sum = 0;
    for &x in input {
        sum += x;
    }
    sum
}

fn bench_comparison(c: &mut Criterion) {
    let data: Vec<i32> = (1..=1000).collect();
    
    c.bench_function("method1", |b| b.iter(|| method1(black_box(&data))));
    c.bench_function("method2", |b| b.iter(|| method2(black_box(&data))));
}

criterion_group!(benches, bench_comparison);
criterion_main!(benches);
```

## Interpréter les Résultats

```
fib 20                  time:   [123.45 ns 124.56 ns 125.67 ns]
```

- **time** = Temps d'exécution moyen
- **ns** = Nanosecondes (très précis!)
- Comparer pour voir quelle méthode est plus rapide

## Official Resources

- [Criterion.rs Documentation](https://github.com/bheisler/criterion.rs)

## Performance Notes

Criterion est excellent pour:
- Mesurer les performances précisément
- Comparer différentes implémentations
- Détecter les régressions de performance
- Optimiser le code basé sur des données réelles

