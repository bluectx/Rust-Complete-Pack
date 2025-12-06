# Gestion d'Erreurs - Introduction

## Learning Objectives

- Comprendre la gestion d'erreurs (c'est important!)
- Utiliser Result<T, E>
- Propager les erreurs avec ?
- Créer des types d'erreurs personnalisés

## Core Explanation

### For Absolute Beginners - C'est Comme un Système de Signalisation! 🚦

Imaginez un **système de signalisation** 🚦:
- **Result** = Vert (Ok) ou Rouge (Err)
- Vous vérifiez le signal → Vous agissez en conséquence
- C'est **super sûr** et **super pratique**!

C'est **exactement** comme la gestion d'erreurs fonctionne! C'est **super important**!

## Schéma Visuel - Gestion d'Erreurs

```
┌─────────────────────────────────────────┐
│  🚦 RESULT = SIGNALISATION 🚦          │
├─────────────────────────────────────────┤
│                                         │
│  Opération                               │
│         │                                │
│    Succès? │ Erreur?                    │
│         │                                │
│    ▼     │     ▼                        │
│  Ok(v)   │  Err(e)                      │
│         │                                │
│  Gestion automatique! ✅                │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Signalisation" - Result est comme un système de signalisation: vert (Ok) = succès, rouge (Err) = erreur, vous agissez en conséquence!

## Code Examples

### Example 1: Result Basique

```rust
fn diviser(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division par zéro!".to_string())
    } else {
        Ok(a / b)
    }
}
```

## Official Resources

- [@official Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

