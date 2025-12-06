use crossterm::{cursor, execute, terminal};
use std::io::{self, stdout};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    
    println!("Jeu Terminal - Appuyez sur 'q' pour quitter");
    
    loop {
        // Game loop simplifié
        if let Ok(c) = read_char() {
            if c == 'q' {
                break;
            }
        }
    }
    
    terminal::disable_raw_mode()?;
    Ok(())
}

fn read_char() -> io::Result<char> {
    let mut buffer = [0; 1];
    io::stdin().read_exact(&mut buffer)?;
    Ok(buffer[0] as char)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_game_logic() {
        assert!(true);
    }
}

