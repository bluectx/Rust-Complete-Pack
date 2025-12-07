// Visibility avec pub in
// Objectif: Restreindre la visibilité à des modules spécifiques

// TODO: Utilisez `pub(in path) fn ...` pour une visibilité limitée....

fn main() {
    // Votre code ici
    // Suivez les instructions du CHALLENGE.md
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