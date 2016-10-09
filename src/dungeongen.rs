


pub fn printStartingAreaForRoll(roll: u32)
{
    if roll > 10 || roll < 1
    {
        return();
    }



    match roll{
       1 => println!("Square, 20x20; Passage on each wall"),
       2 => println!( "Square 20x20; door on two walls, passage in third wall"),
       3 => println!("Square, 40 x 40 ft; doors on three walls"),
       4 => println!("Rectangle, 80 x20ft with a row of pillars down the middle; two passages leading from each long wall, doors on each short wall"),
       5 => println!("Rectangle, 20 x 40 ft.; passage on each wall"),
       6 => println!("Cicle, 40ft. diameter; one passage at each cardinal direction"),
       7 => println!("Cicle, 40ft. diameter; one passage at each cardinal direction; well in middle of room (might lead down to lower level)"),
       8 => println!("Square, 20 x 20ft.; door on two walls, passage on third wall, secret door on fourth wall"),
       9 => println!("Passage, 10 ft. wide; T intersection"),
       10 => println!("Passage, 10 ft. wide; four way intersection"),
       none => return(),
    }
   

}



pub fn printPassage(roll: u32)
{
       match roll{
       1 => println!("Continue Straight 30 ft., no doors or side passages"),
       2 => println!( "Continue Straight 30 ft., no doors or side passages"),
       3 => println!("Continue straight 20 ft. door to the right and then 10 ft. ahead"),
       4 => println!("Continue straight 20 ft., door to the left, then an additional 10 ft. ahead"),
       5 => println!("Continue Straight 20 ft., passage ends in a door."),
       6 => println!("Continue Straight 20ft., side passage to the right, then an additional 10 ft. ahead"),
       7 => println!("Continue Straight 20ft., side passage to the right, then an additional 10 ft. ahead"),
       8 => println!("Continue Straight 20ft., side passage to the left, then an additional 10 ft. ahead"),
       9 => println!("Continue Straight 20ft., side passage to the left, then an additional 10 ft. ahead"),
       10 => println!("Continue straight ahead 20ft., comes to a dead end; 10 percent chance of a secret door."),
       11 => println!("Continue straight ahead 10ft., then the passage turns left and continues 10 ft."),
       12 => println!("Continue straight ahead 10ft., then the passage turns left and continues 10 ft."),
       13 => println!("Continue straight ahead 10ft., then the passage turns right and continues 10 ft."),
       14 => println!("Continue straight ahead 10ft., then the passage turns right and continues 10 ft."),
       15 => println!("Chamber"),
       16 => println!("Chamber"),
       17 => println!("Chamber"),
       18 => println!("Chamber"),
       19 => println!("Chamber"),
       20 => println!("Stairs"),
       none => return(),
    }
}

pub fn printChamber(roll: u32)
{
    match roll{
        1  => println!("Normal Chamber: 20 X 20"), 
        2  => println!("Normal Chamber: 20 X 20"), 
        3  => println!("Normal Chamber: 30 X 30"), 
        4  => println!("Normal Chamber: 30 X 30"), 
        5  => println!("Normal Chamber: 40 X 40"), 
        6  => println!("Normal Chamber: 40 X 40"), 
        7  => println!("Normal Chamber: 20 X 30"), 
        8  => println!("Normal Chamber: 20 X 30"), 
        9  => println!("Normal Chamber: 20 X 40"), 
        10 => println!("Normal Chamber: 30 X 40"), 
        11 => println!("Normal Chamber: 30 X 40"), 
        12 => println!("Normal Chamber: 30 X 40"), 
        13 => println!("Large Chamber:  40 X 50"), 
        14 => println!("Large Chamber:  40 X 50"), 
        15 => println!("Large Chamber:  50 X 80"), 
        16 => println!("Normal Chamber: Cicle 30ft. diameter"), 
        17 => println!("Large Chamber:  Cicle 50ft. diameter"), 
        18 => println!("Large Chamber:  Octagon 40 x 40"), 
        19 => println!("Large Chamber:  Octagon 60 x 60"), 
        20 => println!("Large Chamber:  Trapezoid 40 x 60"),
        none => return(),
    }
}

pub fn printChamberExits(roll: u32)
{
    match roll{
    1 => println!("Chamber Exits: N: 0 , L 0"),
    2 => println!("Chamber Exits: N: 0 , L 0"),
    3 => println!("Chamber Exits: N: 0 , L 0"),
    4 => println!("Chamber Exits: N: 0 , L 1"),
    5 => println!("Chamber Exits: N: 0 , L 1"),
    6 => println!("Chamber Exits: N: 1 , L 1"),
    7 => println!("Chamber Exits: N: 1 , L 1"),
    8 => println!("Chamber Exits: N: 1 , L 1"),
    9 => println!("Chamber Exits: N: 1 , L 2"),
    10 => println!("Chamber Exits: N: 1 , L 2"),
    11 => println!("Chamber Exits: N: 1 , L 2"),
    12 => println!("Chamber Exits: N: 2 , L 2"),
    13 => println!("Chamber Exits: N: 2 , L 2"),
    14 => println!("Chamber Exits: N: 2 , L 3"),
    15 => println!("Chamber Exits: N: 2 , L 3"),
    16 => println!("Chamber Exits: N: 3 , L 3"),
    17 => println!("Chamber Exits: N: 3 , L 3"),
    18 => println!("Chamber Exits: N: 3 , L 4"),
    19 => println!("Chamber Exits: N: 4 , L 5"),
    20 => println!("Chamber Exits: N: 4 , L 6"),
    none => return(),
}
}


