use super::*;

mod starting_classes {
    use super::StartingClassType;

    pub struct StartingClass {
        pub name: String,
        pub level: i32,
        pub stats: [i32; 8],
        pub total: i32,
    }

    impl StartingClass {
        pub fn new(name: String, level: i32, stats: [i32; 8], total: i32) -> StartingClass {
            StartingClass {
                name,
                level,
                stats,
                total,
            }
        }
    }

    pub fn from_type(starting_class: StartingClassType) -> StartingClass {
        match starting_class {
            StartingClassType::Hero => {
                StartingClass::new(String::from("Hero"), 7, [14, 9, 12, 16, 9, 7, 8, 11], 86)
            }
            StartingClassType::Bandit => {
                StartingClass::new(String::from("Bandit"), 5, [10, 11, 10, 9, 13, 9, 8, 14], 84)
            }
            StartingClassType::Astrologer => StartingClass::new(
                String::from("Astrologer"),
                6,
                [9, 15, 9, 8, 12, 16, 7, 9],
                85,
            ),
            StartingClassType::Warrior => StartingClass::new(
                String::from("Warrior"),
                8,
                [11, 12, 11, 10, 16, 10, 8, 9],
                87,
            ),
            StartingClassType::Prisoner => StartingClass::new(
                String::from("Prisoner"),
                9,
                [11, 12, 11, 11, 14, 14, 6, 9],
                88,
            ),
            StartingClassType::Confessor => StartingClass::new(
                String::from("Confessor"),
                10,
                [10, 13, 10, 12, 12, 9, 14, 9],
                89,
            ),
            StartingClassType::Wretch => StartingClass::new(
                String::from("Wretch"),
                1,
                [10, 10, 10, 10, 10, 10, 10, 10],
                80,
            ),
            StartingClassType::Vagabond => StartingClass::new(
                String::from("Vagabond"),
                9,
                [15, 10, 11, 14, 13, 9, 9, 7],
                88,
            ),
            StartingClassType::Prophet => StartingClass::new(
                String::from("Prophet"),
                7,
                [10, 14, 8, 11, 10, 7, 16, 10],
                86,
            ),
            StartingClassType::Samurai => StartingClass::new(
                String::from("Samurai"),
                6,
                [12, 11, 13, 12, 15, 9, 8, 8],
                88,
            ),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StatList {
    pub level: i32,
    pub vigor: i32,
    pub mind: i32,
    pub endurance: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub faith: i32,
    pub arcane: i32,
    pub class: StartingClassType,
}

impl StatList {
    pub fn from_slice(
        stats_list: [i32; 8],
        level: i32,
        class: StartingClassType,
    ) -> Result<StatList, &'static str> {
        // this needs to verify

        Ok(StatList {
            level,
            vigor: stats_list[0],
            mind: stats_list[1],
            endurance: stats_list[2],
            strength: stats_list[3],
            dexterity: stats_list[4],
            intelligence: stats_list[5],
            faith: stats_list[6],
            arcane: stats_list[7],
            class,
        })
    }

    pub fn from_starting_class(starting_class: StartingClassType) -> StatList {
        let starting_class_vals = starting_classes::from_type(starting_class);
        StatList::from_slice(
            starting_class_vals.stats,
            starting_class_vals.level,
            starting_class,
        )
        .unwrap()
    }

    pub fn unspent_levels(&mut self) -> Result<i32, &'static str> {
        let starting_class_vals = starting_classes::from_type(self.class);

        let stat_sum = [
            self.vigor,
            self.mind,
            self.endurance,
            self.strength,
            self.dexterity,
            self.intelligence,
            self.faith,
            self.arcane,
        ]
        .iter()
        .fold(0, |acc, stat_val| acc + stat_val);

        let spent_levels = self.level - starting_class_vals.level;
        let levels_allocated = stat_sum - starting_class_vals.total;

        if levels_allocated > spent_levels {
            self.level = stat_sum;
            return Err("Level value is incorrect");
        }

        Ok(spent_levels - levels_allocated)
    }

    // check if the stats meet the requirements of the weapon
    pub fn check_weapon_requirements(&self, weapon: &weapons::Weapon) -> bool {
        for stat in [
            Scaling::Str,
            Scaling::Dex,
            Scaling::Int,
            Scaling::Fai,
            Scaling::Arc,
        ] {
            let stat_val = self[stat];
            if stat_val < weapon.get_required_scaling_stat(stat) {
                return false;
            }
        }

        true
    }
}

impl Index<Scaling> for StatList {
    type Output = i32;

    fn index(&self, index: Scaling) -> &Self::Output {
        match index {
            Scaling::Str => &self.strength,
            Scaling::Dex => &self.dexterity,
            Scaling::Int => &self.intelligence,
            Scaling::Fai => &self.faith,
            Scaling::Arc => &self.arcane,
        }
    }
}

impl IndexMut<Scaling> for StatList {
    fn index_mut(&mut self, index: Scaling) -> &mut Self::Output {
        match index {
            Scaling::Str => &mut self.strength,
            Scaling::Dex => &mut self.dexterity,
            Scaling::Int => &mut self.intelligence,
            Scaling::Fai => &mut self.faith,
            Scaling::Arc => &mut self.arcane,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_unspent_levels() {
        let mut stats = dbg!(StatList::from_slice(
            [60, 15, 40, 80, 14, 15, 6, 9],
            160,
            StartingClassType::Prisoner,
        )
        .unwrap());

        assert_eq!(stats.unspent_levels().unwrap(), 0);
    }

    #[test]
    fn invalid_unspent_levels() {
        let mut stats =
            dbg!(
                StatList::from_slice([50, 15, 30, 50, 14, 15, 6, 9], 40, StartingClassType::Hero,)
                    .unwrap()
            );

        assert_eq!(
            stats.unspent_levels().err(),
            Some("Level value is incorrect")
        );
    }

    #[test]
    fn meets_weapon_requirements_false() {
        let stats = StatList::from_starting_class(StartingClassType::Wretch);
        let weapon = weapons::Weapon::from_data("Moonveil", 3).expect("failed to get weapon");
        assert!(!stats.check_weapon_requirements(&weapon));
    }

    #[test]
    fn meets_weapon_requirements_true() {
        let stats =
            StatList::from_slice([50, 15, 30, 50, 50, 50, 6, 9], 40, StartingClassType::Hero)
                .unwrap();
        let weapon = weapons::Weapon::from_data("Moonveil", 3).expect("failed to get weapon");
        assert!(stats.check_weapon_requirements(&weapon));
    }

    #[test]
    fn test_stat_list_index() {
        let stats =
            StatList::from_slice([50, 15, 30, 50, 50, 50, 6, 9], 40, StartingClassType::Hero)
                .unwrap();
        assert_eq!(stats[Scaling::Str], 50);
        assert_ne!(stats[Scaling::Fai], 50);
    }
}
