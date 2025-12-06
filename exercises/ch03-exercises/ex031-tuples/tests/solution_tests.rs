#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tuple_creation() {
        let (nom, age, ville) = obtenir_infos();
        assert!(!nom.is_empty());
        assert!(age > 0);
        assert!(!ville.is_empty());
    }
}

