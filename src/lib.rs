#![allow(dead_code)]

use csv;
use std::collections::HashMap;
use std::error::Error;
use std::ops::{Index, IndexMut};
use std::path::Path;
use std::fmt::Display;

mod ar_calculator;
mod optimizers;
mod stats;
mod weapons;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CoreStat {
    Vig,
    Mnd,
    End,
    Str,
    Dex,
    Int,
    Fai,
    Arc,
}

impl CoreStat {
    fn iter_all() -> impl Iterator<Item = CoreStat> {
        [
            CoreStat::Vig,
            CoreStat::Mnd,
            CoreStat::End,
            CoreStat::Str,
            CoreStat::Dex,
            CoreStat::Int,
            CoreStat::Fai,
            CoreStat::Arc,
        ]
        .iter()
        .copied()
    }

    fn iter_scalings() -> impl Iterator<Item = CoreStat> {
        [
            CoreStat::Str,
            CoreStat::Dex,
            CoreStat::Int,
            CoreStat::Fai,
            CoreStat::Arc,
        ]
        .iter()
        .copied()
    }
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

type Levels = i32;
