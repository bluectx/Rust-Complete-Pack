// TODO: Corriger ce code pour qu'il compile
// Le problème: ownership de s1 est déplacé vers s2

fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 est déplacé vers s2
    
    // TODO: Comment utiliser s1 et s2 sans erreur?
    // Option 1: Utiliser seulement s2
    // Option 2: Cloner s1 avant le move
    // Option 3: Utiliser borrowing
    
    println!("s1: {}", s1);  // ERREUR: s1 n'est plus valide
    println!("s2: {}", s2);
}

