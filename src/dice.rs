extern crate rand;

use self::rand::Rng;
use self::rand::thread_rng;

pub fn rollD10() -> u32
{
    let mut rng = thread_rng();
    let num: u32 = rng.gen_range(0, 10);
    return num + 1;
}

pub fn rollD12() -> u32
{
    let mut rng = thread_rng();
    let num: u32 = rng.gen_range(0, 12);
    return num + 1;
}

pub fn rollD20() -> u32{
    let mut rng = thread_rng();
    let num: u32 = rng.gen_range(0, 20);
    return num + 1;
}