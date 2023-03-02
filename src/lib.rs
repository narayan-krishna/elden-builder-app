#![allow(dead_code)]

use csv;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

pub mod ar_calculator;

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
    pub fn from_vec(stats_list: Vec<i32>) {
        todo!()
    }

    pub fn from_starting_class(starting_class: StartingClass) -> StatList {
        todo!()
    }

    pub fn optimize(&mut self, weapon: &Weapon, current_level: i32) {
        todo!()
    }

    pub fn optimize_from_starting_class(weapon: &Weapon) {
        todo!()
    }
}

#[derive(Debug)]
pub struct Weapon {
    name: String,
    upgrade_lvl: i32,
    reinforce_param_id: i32,
    attack_element_correct_id: i32,
    stat_vals: Vec<f32>,
    modifiers: Vec<f32>,
    required_stats: Vec<i32>,
}

impl Weapon {
    pub fn new(
        name: &str,
        reinforce_param_id: i32,
        stat_vals: Vec<f32>,
        modifiers: Vec<f32>,
        required_stats: Vec<i32>,
    ) -> Weapon {
        Weapon {
            name: name.to_string(),
            upgrade_lvl: 0,
            reinforce_param_id,
            attack_element_correct_id: 0,
            stat_vals,
            modifiers,
            required_stats,
        }
    }

    fn get_attack_stat(&self, attack: Attack) -> f32 {
        match attack {
            Attack::Physical => self.stat_vals[0] * self.modifiers[0],
            Attack::Magic => self.stat_vals[1] * self.modifiers[1],
            Attack::Fire => self.stat_vals[2] * self.modifiers[2],
            Attack::Lightning => self.stat_vals[3] * self.modifiers[3],
            Attack::Holy => self.stat_vals[4] * self.modifiers[4],
            Attack::Stamina => self.stat_vals[5] * self.modifiers[5],
        }
    }

    fn get_scaling_stat(&self, scaling: Scaling) -> f32 {
        match scaling {
            Scaling::Str => self.stat_vals[6] * self.modifiers[6],
            Scaling::Dex => self.stat_vals[7] * self.modifiers[7],
            Scaling::Int => self.stat_vals[8] * self.modifiers[8],
            Scaling::Fai => self.stat_vals[9] * self.modifiers[9],
            Scaling::Arc => self.stat_vals[10] * self.modifiers[10],
        }
    }

    fn get_required_scaling_stat(&self, scaling_stat: Scaling) -> i32 {
        match scaling_stat {
            Scaling::Str => self.required_stats[0],
            Scaling::Dex => self.required_stats[1],
            Scaling::Int => self.required_stats[2],
            Scaling::Fai => self.required_stats[3],
            Scaling::Arc => self.required_stats[4],
        }
    }

    /// change the upgrade level of the weapon (indirectly changing the damage modifiers)
    pub fn upgrade_weapon(&mut self, upgrade_lvl: i32) {
        self.upgrade_lvl = upgrade_lvl;
        self.modifiers =
            ar_calculator::get_reinforce_param_modifier(self.reinforce_param_id + upgrade_lvl)
                .unwrap();
    }

    /// build a weapon from the raw weapon data, given a name
    pub fn build_from_data(weapon_name: &str, upgrade_lvl: i32) -> Result<Weapon, Box<dyn Error>> {
        let path = Path::new("csv_data/RawData.csv");
        let mut rdr = csv::Reader::from_path(path)?;
        let mut stat_vals: Vec<f32> = Vec::new();
        let mut required_stats: Vec<i32> = Vec::new();

        let name_pos = 1;
        let reinforce_param_pos = 2;
        let attack_element_correct_id_pos = 26;
        let stat_range = (3, 14);
        let required_stat_range = (29, 34);

        let mut reinforce_param_id = 0;
        let mut attack_element_correct_id = 0;

        for result in rdr.records() {
            let record = result?;
            if record.get(name_pos).unwrap() == weapon_name.to_string() {
                reinforce_param_id = record.get(reinforce_param_pos).unwrap().parse()?;
                attack_element_correct_id =
                    record.get(attack_element_correct_id_pos).unwrap().parse()?;
                stat_vals = (stat_range.0..stat_range.1)
                    .map(|i| {
                        record
                            .get(i)
                            .unwrap()
                            .parse()
                            .expect("failed to translate entry to number")
                    })
                    .collect();

                required_stats = (required_stat_range.0..required_stat_range.1)
                    .map(|i| {
                        record
                            .get(i)
                            .unwrap()
                            .parse()
                            .expect("failed to translate entry to number")
                    })
                    .collect();
                break;
            }
        }

        if rdr.is_done() {
            return Err("failed to find weapon".into());
        }

        let modifiers =
            ar_calculator::get_reinforce_param_modifier(reinforce_param_id + upgrade_lvl)?;

        Ok(Weapon {
            name: weapon_name.to_string(),
            reinforce_param_id,
            attack_element_correct_id,
            upgrade_lvl,
            stat_vals,
            modifiers,
            required_stats,
        })
    }
}
