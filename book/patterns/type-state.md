# Type-State Pattern - États à la Compilation! 🎯

## Learning Objectives

- Comprendre le type-state pattern (c'est puissant!)
- Créer des types avec états à la compilation
- Garantir les invariants à la compilation
- Voir les cas d'usage

## Core Explanation

### For Absolute Beginners - C'est Comme un Passeport avec Visas! 🛂

Imaginez un **passeport** 🛂 avec des visas:
- **Type-state** = Le passeport change d'état (non-vérifié → vérifié)
- Le compilateur vérifie que vous avez le bon visa avant d'entrer!
- Impossible d'oublier une étape!

C'est **exactement** comme le type-state pattern fonctionne! C'est **super sûr**!

## Schéma Visuel - Type-State

```
┌─────────────────────────────────────────┐
│  🛂 TYPE-STATE = PASSEPORT 🛂          │
├─────────────────────────────────────────┤
│                                         │
│  User<Unverified>                       │
│         │                                │
│         ▼ verify()                      │
│  User<Verified>                         │
│         │                                │
│         ▼ access_resource()             │
│  ✅ Accès autorisé!                     │
│                                         │
│  États vérifiés à la compilation! ✅    │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Passeport" - Le type-state pattern est comme un passeport: vous devez avoir le bon visa (état) avant d'entrer, vérifié à la compilation!

## Code Examples

### Example 1: Type-State Basique

```rust
struct Unverified;
struct Verified;

struct User<State> {
    name: String,
    _state: std::marker::PhantomData<State>,
}

impl User<Unverified> {
    fn new(name: String) -> Self {
        User {
            name,
            _state: std::marker::PhantomData,
        }
    }
    
    fn verify(self) -> User<Verified> {
        User {
            name: self.name,
            _state: std::marker::PhantomData,
        }
    }
}

impl User<Verified> {
    fn access_resource(&self) {
        println!("Accès autorisé pour {}", self.name);
    }
}

fn main() {
    let user = User::<Unverified>::new("Alice".to_string());
    // user.access_resource();  // ERREUR: pas encore vérifié
    
    let verified = user.verify();
    verified.access_resource();  // OK: vérifié
}
```

## Avantages

- **Sécurité à la compilation** : États vérifiés par le type system
- **Pas de runtime overhead** : Vérification à la compilation
- **Impossible d'oublier** : Le compilateur force les transitions

## Official Resources

- [Rust Design Patterns - Type State](https://rust-unofficial.github.io/patterns/patterns/behavioural/state.html)

