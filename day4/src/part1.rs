
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn exec() -> std::io::Result<()> {
    
    let mut sum_totally_recovered = 0 ;

    // File reading

    let file = File::open("input.txt")? ;
    let reader = BufReader::new(file) ;

    for line in reader.lines() {

        let line = line? ;

        // On récupère les paires
        let mut tmp = line.split(',') ;
        let elf1 = tmp.next().unwrap() ;
        let elf2 = tmp.next().unwrap() ;

        
        // Autre méthode pour récupèrer les parties d'un split
        let parts: Vec<&str> = elf1.split('-').collect() ;

        // On génére un tableau de la forme [a a+1 ... b] pour les 2 elfes de la paire

        let a: i32 = parts[0].parse().unwrap() ;
        let b: i32 = parts[1].parse().unwrap() ;

        let mut assignement1: Vec<i32> = Vec::new() ;
        for i in a..=b {
            assignement1.push(i) ;
        }

        let parts: Vec<&str> = elf2.split('-').collect() ;
        let a: i32 = parts[0].parse().unwrap() ;
        let b: i32 = parts[1].parse().unwrap() ;

        let mut assignement2: Vec<i32> = Vec::new() ;
        for i in a..=b {
            assignement2.push(i) ;
        }

        // On vérifie l'inclusion de 1 dans 2 ou de 2 dans 1

        let set1: HashSet<i32> = assignement1.iter().cloned().collect() ;
        let set2: HashSet<i32> = assignement2.iter().cloned().collect() ;

        if set1.is_subset(&set2) || set2.is_subset(&set1) {
            sum_totally_recovered += 1 ;
        }


    }

    println!("Le résultat est : {sum_totally_recovered}") ;

    Ok(())

}
