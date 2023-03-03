#![allow(dead_code)]

use csv;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

mod ar_calculator;
mod weapon;
mod stats;

pub enum Scaling {
    Str,
    Dex,
    Int,
    Fai,
    Arc,
}

pub enum Attack {
    Physical,
    Magic,
    Fire,
    Lightning,
    Holy,
    Stamina,
}

pub enum StartingClass {
    Hero,
    Bandit,
    Astrologer,
    Warrior,
    Prisoner,
    Confessor,
    Wretch,
    Vagabond,
    Prophet,
    Samurai,
}
