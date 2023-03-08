#![allow(dead_code)]

use csv;
use std::collections::HashMap;
use std::error::Error;
use std::ops::{Index, IndexMut};
use std::path::Path;

mod ar_calculator;
mod optimizers;
mod stats;
mod weapons;

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StartingClassType {
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
