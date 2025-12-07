// Ownership et return
// Objectif: Retourner une valeur pour transférer l'ownership

// TODO: Écrivez une fonction qui retourne une valeur

fn ma_fonction(/* TODO: paramètres */) -> /* TODO: type de retour */ {
    // Votre code ici
    // Retournez une valeur (sans ; pour une expression)
}

fn main() {
    let resultat = ma_fonction(/* TODO: arguments */);
    println!("Résultat: {}", resultat);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise() {
        // TODO: Ajoutez vos tests ici
        // Exemple: assert_eq!(ma_fonction(), valeur_attendue);
    }
}