
mod dice;
mod dungeongen;

use std::io;
use std::env;

fn main() {

let args: Vec<_> = env::args().collect();

 if let Some(arg1) = env::args().nth(1) {
     
     match arg1.as_ref() {
         "Passage" => roll_passage(),
         "-P" => roll_passage(),
         "-p" => roll_passage(),
         "Chamber" => roll_chamber(),
         "-C" => roll_chamber(),
         "-c" => roll_chamber(),
         "-S" => roll_start_area(),
         "-s" => roll_start_area(),
         none => println!("Please Provide a valid Option"),
     }
}


    
}


fn roll_passage()
{
 let dice_roll =  dice::rollD20();
 dungeongen::printPassage(dice_roll);
}

fn roll_start_area()
{
     let dice_roll =  dice::rollD10();
     dungeongen::printStartingAreaForRoll(dice_roll);
}

fn roll_chamber()
{
    let chamber_roll = dice::rollD20();
    dungeongen::printChamber(chamber_roll);

     let entrance_roll = dice::rollD20();
     dungeongen::printChamberExits(entrance_roll);
}