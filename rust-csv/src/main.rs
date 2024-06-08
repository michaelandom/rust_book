use csv;

use std::error::Error;



fn reader_csv_file(path: &str) -> Result<(), Box<dyn Error>>{

    let mut reader= csv::Reader::from_path(path)?;


    for result in reader.records(){
        let record = result?;
        println!("{:#?}", record); 
    }
    
    Ok(())

}
fn main() {


    let mut path_value = String::new();
    println!("Enter the CSV file:");
    std::io::stdin().read_line(&mut path_value).expect("some thing error");

    use std::time::Instant;
    let now = Instant::now();
    if let Err(e) = reader_csv_file(path_value.trim()) {
        eprintln!("{}",e);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
} 
