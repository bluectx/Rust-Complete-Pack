// TODO: Créer une fonction qui retourne un tuple (nom, âge, ville)
// Appeler cette fonction et afficher les trois valeurs

fn obtenir_infos() -> (String, u32, String) {
    // TODO: Retourner un tuple avec vos informations
    ("Alice".to_string(), 30, "Paris".to_string())
}

fn main() {
    // TODO: Appeler obtenir_infos() et afficher les valeurs
    let (nom, age, ville) = obtenir_infos();
    println!("Nom: {}, Âge: {}, Ville: {}", nom, age, ville);
}

