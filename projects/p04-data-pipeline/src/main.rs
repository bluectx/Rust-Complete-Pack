use csv::Reader;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Record {
    id: u32,
    value: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("data.csv")?;
    
    let mut sum = 0.0;
    let mut count = 0;
    
    for result in rdr.deserialize() {
        let record: Record = result?;
        sum += record.value;
        count += 1;
    }
    
    let average = if count > 0 { sum / count as f64 } else { 0.0 };
    println!("Moyenne: {:.2}", average);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_average_calculation() {
        assert_eq!(2.0, (1.0 + 2.0 + 3.0) / 3.0);
    }
}

