use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn exec() -> std::io::Result<()> {

    // HashMap to handle the strategy (in particular the score)

    let hm = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3)]
    ) ;

    let answer = HashMap::from([
        ("A", "Y"),
        ("B", "Z"),
        ("C", "X")]
    ) ;

    let mut sum_scores = 0 ;

    // File reading

    let file = File::open("input.txt")? ;
    let reader = BufReader::new(file) ;

    for line in reader.lines() {

        let line = line? ;

        let mut tmp = line.split(' ') ;
        let elf = tmp.next().unwrap() ;
        let player = tmp.next().unwrap() ;

        // On sait désormais quel sera le score de la partie, il faut juste connaitre le score de ce que l'on va jouer
        let mut result = 0 ;
        let score ;

        if player == "Z" {

            result = 6 ;
            score = *hm.get(answer.get(elf).unwrap()).unwrap() ;

        } else if player == "Y" {

            result = 3 ;

            score = match elf {
                "A" => *hm.get("X").unwrap(),
                "B" => *hm.get("Y").unwrap(),
                "C" => *hm.get("Z").unwrap(),
                _ => panic!("Error with the input !")

            } ;

        } else {

            score = match elf {
                "A" => *hm.get("Z").unwrap(),
                "B" => *hm.get("X").unwrap(),
                "C" => *hm.get("Y").unwrap(),
                _ => panic!("Error with the input !")

            } ;

        }

        sum_scores += score + result ;

    }

    println!("Le résultat est : {sum_scores}") ;

    Ok(())

}
