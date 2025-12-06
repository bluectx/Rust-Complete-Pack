use clap::Parser;

#[derive(Parser)]
#[command(name = "hello-world")]
#[command(about = "Un programme Hello World qui lit un nom depuis les arguments CLI")]
struct Args {
    #[arg(help = "Le nom à saluer")]
    name: Option<String>,
}

fn main() {
    let args = Args::parse();
    
    let name = args.name.as_deref().unwrap_or("Monde");
    
    println!("Bonjour, {}!", name);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_greeting_with_name() {
        let args = Args {
            name: Some("Alice".to_string()),
        };
        // Test que le nom est correctement stocké
        assert_eq!(args.name, Some("Alice".to_string()));
    }
    
    #[test]
    fn test_greeting_without_name() {
        let args = Args {
            name: None,
        };
        // Test que le nom par défaut sera utilisé
        assert_eq!(args.name, None);
    }
    
    #[test]
    fn test_default_name() {
        let name = None::<String>.as_deref().unwrap_or("Monde");
        assert_eq!(name, "Monde");
    }
}

