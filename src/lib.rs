#![allow(dead_code)]

use csv;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

pub mod ar_calculator;
pub mod weapon;

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

#[derive(Debug)]
pub struct StatList {
    level: i32,
    vigor: i32,
    mind: i32,
    endurance: i32,
    strength: i32,
    dexterity: i32,
    intelligence: i32,
    faith: i32,
    arcane: i32,
}

impl StatList {
    pub fn from_vec(stats_list: Vec<i32>) -> Result<StatList, Box<dyn Error>> {
        if stats_list.len() != 8 {
            return Err("error".into());
        }

        Ok(StatList {
            level: stats_list[0],
            vigor: stats_list[1],
            mind: stats_list[2],
            endurance: stats_list[3],
            strength: stats_list[4],
            dexterity: stats_list[5],
            intelligence: stats_list[6],
            faith: stats_list[7],
            arcane: stats_list[8],
        })
    }

    pub fn from_starting_class(starting_class: StartingClass) -> StatList {
        todo!()
    }

    pub fn optimize(&mut self, weapon: &weapon::Weapon, current_level: i32) {
        todo!()
    }

    pub fn optimize_from_starting_class(weapon: &weapon::Weapon) {
        todo!()
    }
}
