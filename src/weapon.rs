use super::*;

#[derive(Debug)]
pub struct Weapon {
    pub name: String,
    pub upgrade_lvl: i32,
    pub reinforce_param_id: i32,
    pub attack_element_correct_id: i32,
    pub max_upgrade: i32,
    stat_vals: Vec<f32>,
    modifiers: Vec<f32>,
    required_stats: Vec<i32>,
}

impl Weapon {
    pub fn new(
        name: &str,
        upgrade_lvl: i32,
        reinforce_param_id: i32,
        attack_element_correct_id: i32,
        max_upgrade: i32,
        stat_vals: Vec<f32>,
        modifiers: Vec<f32>,
        required_stats: Vec<i32>,
    ) -> Weapon {
        Weapon {
            name: name.to_string(),
            upgrade_lvl,
            reinforce_param_id,
            attack_element_correct_id,
            max_upgrade,
            stat_vals,
            modifiers,
            required_stats,
        }
    }

    pub fn get_attack_stat(&self, attack: Attack) -> f32 {
        match attack {
            Attack::Physical => self.stat_vals[0] * self.modifiers[0],
            Attack::Magic => self.stat_vals[1] * self.modifiers[1],
            Attack::Fire => self.stat_vals[2] * self.modifiers[2],
            Attack::Lightning => self.stat_vals[3] * self.modifiers[3],
            Attack::Holy => self.stat_vals[4] * self.modifiers[4],
            Attack::Stamina => self.stat_vals[5] * self.modifiers[5],
        }
    }

    pub fn get_scaling_stat(&self, scaling: Scaling) -> f32 {
        match scaling {
            Scaling::Str => self.stat_vals[6] * self.modifiers[6],
            Scaling::Dex => self.stat_vals[7] * self.modifiers[7],
            Scaling::Int => self.stat_vals[8] * self.modifiers[8],
            Scaling::Fai => self.stat_vals[9] * self.modifiers[9],
            Scaling::Arc => self.stat_vals[10] * self.modifiers[10],
        }
    }

    pub fn get_required_scaling_stat(&self, scaling_stat: Scaling) -> i32 {
        match scaling_stat {
            Scaling::Str => self.required_stats[0],
            Scaling::Dex => self.required_stats[1],
            Scaling::Int => self.required_stats[2],
            Scaling::Fai => self.required_stats[3],
            Scaling::Arc => self.required_stats[4],
        }
    }

    /// change the upgrade level of the weapon to given level (indirectly changing the damage modifiers)
    pub fn upgrade_weapon(&mut self, upgrade_lvl: i32) -> Result<(), Box<dyn Error>> {
        if upgrade_lvl > self.max_upgrade {
            eprintln!("upgrade level is too high!");
            return Err("the upgrade level entered exceeds the max upgrade level".into());
        }

        self.upgrade_lvl = upgrade_lvl;

        match ar_calculator::get_reinforce_param_modifier(self.reinforce_param_id + upgrade_lvl) {
            Ok(modifiers) => {
                self.modifiers = modifiers;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// build a weapon from the raw weapon data, given a name
    pub fn from_data(weapon_name: &str, upgrade_lvl: i32) -> Result<Weapon, Box<dyn Error>> {
        let path = Path::new("csv_data/RawData.csv");
        let mut rdr = csv::Reader::from_path(path)?;
        let mut stat_vals: Vec<f32> = Vec::new();
        let mut required_stats: Vec<i32> = Vec::new();

        let name_pos = 1;
        let reinforce_param_pos = 2;
        let attack_element_correct_id_pos = 26;
        let max_upgrade_pos = 28;
        let stat_range = (3, 14);
        let required_stat_range = (29, 34);

        let mut reinforce_param_id = 0;
        let mut attack_element_correct_id = 0;
        let mut max_upgrade = 0;

        for result in rdr.records() {
            let record = result?;
            if record.get(name_pos).unwrap() == weapon_name.to_string() {
                reinforce_param_id = record.get(reinforce_param_pos).unwrap().parse()?;
                attack_element_correct_id =
                    record.get(attack_element_correct_id_pos).unwrap().parse()?;
                max_upgrade = record.get(max_upgrade_pos).unwrap().parse()?;
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
            upgrade_lvl,
            reinforce_param_id,
            attack_element_correct_id,
            max_upgrade,
            stat_vals,
            modifiers,
            required_stats,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ar_calculator;

    // #[test]
    // fn build_weapon() {
    //     let weapon_1 = Weapon {
    //         name: String::from("Ruins Greatsword"),
    //         reinforce_param_id: 2000,
    //         attack_element_correct_id: 100,
    //         upgrade_lvl: 5,
    //         stat_vals: ,
    //         modifiers,
    //         required_stats,
    //     };
    // }

    #[test]
    fn invalid_name() {
        let ruins_gs_0 = Weapon::from_data("fiaonwe", 0);
        assert!(ruins_gs_0.is_err());
    }

    #[test]
    fn invalid_upgrade_level() {
        let ruins_gs_0 = Weapon::from_data("Ruins Greatsword", 12);
        assert!(ruins_gs_0.is_err());
    }

    #[test]
    fn invalid_weapon_upgrade_modification() {
        let stats = stats::StatList {
            level: 10,
            vigor: 10,
            mind: 10,
            endurance: 10,
            strength: 50,
            dexterity: 14,
            intelligence: 40,
            faith: 10,
            arcane: 10,
        };

        let mut ruins_gs_5 = Weapon::from_data("Ruins Greatsword", 5).unwrap();
        let ruins_gs_5_ar = ar_calculator::calculate_ar(&ruins_gs_5, &stats).unwrap();
        assert_eq!(ruins_gs_5_ar, 487.0);

        let upgrade = ruins_gs_5.upgrade_weapon(12);
        assert!(upgrade.is_err());
        assert_eq!(ruins_gs_5.upgrade_lvl, 5);
    }

    #[test]
    fn valid_weapon_upgrade_modification() {
        let stats = stats::StatList {
            level: 10,
            vigor: 10,
            mind: 10,
            endurance: 10,
            strength: 50,
            dexterity: 14,
            intelligence: 40,
            faith: 10,
            arcane: 10,
        };

        let mut ruins_gs_5 = Weapon::from_data("Ruins Greatsword", 5).unwrap();
        let ruins_gs_5_ar = ar_calculator::calculate_ar(&ruins_gs_5, &stats).unwrap();
        assert_eq!(ruins_gs_5_ar, 487.0);

        ruins_gs_5.upgrade_weapon(10).unwrap();
        let ruins_gs_10_ar = ar_calculator::calculate_ar(&ruins_gs_5, &stats).unwrap();
        assert_eq!(ruins_gs_10_ar, 777.0);
    }
}
