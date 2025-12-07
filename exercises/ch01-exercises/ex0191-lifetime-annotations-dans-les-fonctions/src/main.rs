// Lifetime annotations dans les fonctions
// Objectif: Annotter explicitement les lifetimes

// TODO: Utilisez les références (&) pour emprunter

fn main() {
    let s = String::from("hello");
    // TODO: Créez une référence &s
    // TODO: Passez-la à une fonction
    // TODO: Utilisez s après (devrait fonctionner!)
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