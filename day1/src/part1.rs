
use std::cmp;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn exec() -> std::io::Result<()> { // ? = Error handling while reading the file
    
    // Open the file in read-only mode
    let file = File::open("calories_input.txt")? ;

    // Create a BufReader object to read the file line by line
    let reader = BufReader::new(file) ;

    // Stock the elf with the maxmimum number of calories
    let mut result = -1 ;
    let mut sum = 0 ;

    // Iterate through the lines
    for line in reader.lines() {

        let line = line? ;

        // Nouveau elfe
        if line == "" {
            result = cmp::max(sum, result) ;
            sum = 0 ;
        }

        // Toujours le même inventaire
        else {

            let tmp = match line.parse::<i32>() {
                Ok(sum) => sum,
                Err(e) => {
                    println!("/!\\ Error while parsing line {line} - {}", e) ;
                    continue ;
                }
            } ;

            sum += tmp ;

        }

    }    

    // Calories max
    println!("Le résultat est : {result}\n") ;

    Ok(())

}
