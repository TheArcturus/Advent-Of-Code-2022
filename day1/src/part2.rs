
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn exec() -> std::io::Result<()> { // ? = Error handling while reading the file
    
    // Open the file in read-only mode
    let file = File::open("calories_input.txt")? ;

    // Create a BufReader object to read the file line by line
    let reader = BufReader::new(file) ;

    // Stock the elf with the maxmimum number of calories
    let mut results : Vec<i32> = Vec::new() ;
    let mut sum = 0 ;

    // Iterate through the lines
    for line in reader.lines() {

        let line = line? ;

        // Nouveau elfe
        if line == "" {
            results.push(sum) ;
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

    // Trie par ordre décroissant puis récupération du top 3
    results.sort_by(|a, b| b.cmp(a)) ; 

    let top1 = results[0] ;
    let top2 = results[1] ;
    let top3 = results[2] ;
    let result = top1+top2+top3 ;

    // Calories max
    println!("Le résultat est : {result}\n") ;

    Ok(())

}
