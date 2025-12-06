# Macros Déclaratives - Raccourcis Magiques! 🎯

## Learning Objectives

- Créer des macros avec macro_rules! (c'est puissant!)
- Comprendre la syntaxe des macros
- Utiliser les patterns
- Voir les exemples courants

## Core Explanation

### For Absolute Beginners - C'est Comme un Raccourci Clavier! ⌨️

Imaginez un **raccourci clavier** ⌨️:
- **Macro** = Un raccourci qui remplace du code
- Vous tapez le raccourci → Le compilateur le remplace!
- C'est **super pratique** pour éviter la répétition!

C'est **exactement** comme les macros fonctionnent! C'est **super puissant**!

## Schéma Visuel - Macros

```
┌─────────────────────────────────────────┐
│  ⌨️ MACROS = RACCOURCIS CLAVIER ⌨️     │
├─────────────────────────────────────────┤
│                                         │
│  Vous tapez:                             │
│  say_hello!()                           │
│                                         │
│  Macro transforme en:                    │
│  println!("Hello, World!");             │
│                                         │
│  C'est automatique! ✨                  │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Raccourci" - Les macros sont comme des raccourcis clavier: vous tapez un raccourci, le compilateur le remplace par le code complet!

## Code Examples

### Example 1: Macro Basique

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!();
}
```

### Example 2: Macro avec Paramètres

```rust
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("Fonction {} appelée", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

fn main() {
    foo();
    bar();
}
```

### Example 3: Macro Répétitive

```rust
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v = vec![1, 2, 3];
    println!("{:?}", v);
}
```

### Example 4: Macro avec Patterns Multiples

```rust
macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}",
                 $left, $right, $left && $right);
    };
    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}",
                 $left, $right, $left || $right);
    };
}

fn main() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}
```

## Fragment Specifiers

```
ident  → Identifiant (nom de variable, fonction)
expr   → Expression
ty     → Type
pat    → Pattern
stmt   → Statement
block  → Bloc de code
item   → Item (fonction, struct, etc.)
```

## Official Resources

- [@official Rust Book - Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)

## Security Notes

Les macros sont expansées à la compilation :
- Pas de runtime overhead
- Vérification à la compilation
- Attention aux injections de code
- Tester exhaustivement
