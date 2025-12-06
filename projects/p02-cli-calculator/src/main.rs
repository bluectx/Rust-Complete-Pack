use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
#[command(name = "calculator")]
#[command(about = "Calculatrice CLI simple")]
struct Args {
    #[arg(help = "Premier nombre")]
    a: f64,
    #[arg(help = "Opération (+, -, *, /)")]
    op: String,
    #[arg(help = "Deuxième nombre")]
    b: f64,
}

fn calculer(a: f64, op: &str, b: f64) -> Result<f64> {
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                anyhow::bail!("Division par zéro impossible");
            }
            Ok(a / b)
        }
        _ => anyhow::bail!("Opération non supportée: {}. Utilisez +, -, *, ou /", op),
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    let resultat = calculer(args.a, &args.op, args.b)
        .with_context(|| format!("Erreur lors du calcul: {} {} {}", args.a, args.op, args.b))?;
    
    println!("{} {} {} = {}", args.a, args.op, args.b, resultat);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_addition() {
        assert_eq!(calculer(5.0, "+", 3.0).unwrap(), 8.0);
    }
    
    #[test]
    fn test_soustraction() {
        assert_eq!(calculer(10.0, "-", 4.0).unwrap(), 6.0);
    }
    
    #[test]
    fn test_multiplication() {
        assert_eq!(calculer(3.0, "*", 4.0).unwrap(), 12.0);
    }
    
    #[test]
    fn test_division() {
        assert_eq!(calculer(15.0, "/", 3.0).unwrap(), 5.0);
    }
    
    #[test]
    fn test_division_par_zero() {
        assert!(calculer(10.0, "/", 0.0).is_err());
    }
    
    #[test]
    fn test_operation_invalide() {
        assert!(calculer(5.0, "%", 3.0).is_err());
    }
}

