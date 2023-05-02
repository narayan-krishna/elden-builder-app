use super::*;
use serde::{Deserialize, Serialize};

/// module provides a starting class struct which holds starting class stats
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

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
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
    /// create any possible statlist from a slice, without worrying about meeting being derived properly from a starting class.
    pub fn from_slice(stats_list: [i32; 8], level: i32, class: StartingClassType) -> StatList {
        // this needs to verify

        let stats = StatList {
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
        };

        stats
    }

    /// create a statlist form a slice, checking that the statlist is valid in the context of the
    /// starting class
    pub fn from_slice_with_class_check(
        stats_list: [i32; 8],
        level: i32,
        class: StartingClassType,
    ) -> Result<StatList, &'static str> {
        // this needs to verify

        let stats = StatList {
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
        };

        stats.check_stats_match_starting_class()?;

        Ok(stats)
    }

    /// build a statlist using starting class module
    pub fn from_starting_class(starting_class: StartingClassType) -> StatList {
        let starting_class_vals = starting_classes::from_type(starting_class);
        // we use from slice here because starting classes shouldn't be checked against themselves
        StatList::from_slice(
            starting_class_vals.stats,
            starting_class_vals.level,
            starting_class,
        )
    }

    /// get the amouint of levels to spend.
    /// check if more levels have been allocated than what is indicated in the current level.
    pub fn get_unspent_levels(&mut self) -> Result<i32, &'static str> {
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

    /// check if the stats meet the requirements of the weapon.
    /// does not affect the statlist.
    pub fn check_weapon_requirements(&self, weapon: &weapons::Weapon) -> bool {
        for stat in CoreStat::iter_scalings() {
            let stat_val = self[stat];
            if stat_val < weapon.get_required_scaling_stat(stat) {
                return false;
            }
        }

        true
    }

    /// check that the statlist is valid in the context of the starting class.
    /// i.e. if the starting class starts at level 7, the statlist cannot have a level less than 7.
    pub fn check_stats_match_starting_class(&self) -> Result<(), &'static str> {
        let err = Err("level of stats is beneath starter level. this cannot happen");
        let starting_class = Self::from_starting_class(self.class);
        if starting_class.level > self.level {
            return err;
        }

        for stat_type in CoreStat::iter_all() {
            if starting_class[stat_type] > self[stat_type] {
                eprintln!("starting class value is too large");
                return err;
            }
        }

        Ok(())
    }

    /// update a stat list so that it meets weapon requirements.
    pub fn push_stats_to_weapon_requirement(
        mut stats: stats::StatList,
        weapon: &weapons::Weapon,
    ) -> Result<stats::StatList, &'static str> {
        let mut unspent = stats.get_unspent_levels()?;

        for stat_type in CoreStat::iter_scalings() {
            let stat_value = &mut stats[stat_type];

            if *stat_value < weapon.get_required_scaling_stat(stat_type) {
                let diff = weapon.get_required_scaling_stat(stat_type) - *stat_value;
                unspent -= diff;
                *stat_value += diff;
            }
        }

        match unspent < 0 {
            true => Err("Not enough levels to meet weapon requirements".into()),
            false => Ok(stats),
        }
    }

    pub fn change_stat(&mut self, stat: CoreStat, val: i32) -> Result<(), &'static str> {
        let starter_class_stats = Self::from_starting_class(self.class);
        if val < starter_class_stats[stat] {
            eprintln!(
                "new val: {:?} is less than starter class stat: {:?}",
                val, starter_class_stats[stat]
            );
            return Err("stat cannot be lower than starting class stat");
        }

        if val > 99 {
            // eprintln!("stat is greater than 99, stat cannot be greater than 99");
            return Err("stat cannot be greater than 99");
        }

        let diff = self[stat] - val;
        self.level -= diff;
        self[stat] = val;

        eprintln!("the new level is: {}", self.level);
        eprintln!(
            "the stat is: {} (used to be: {})",
            self[stat],
            self[stat] + diff
        );

        Ok(())
    }

    pub fn change_level(&mut self, val: i32) -> Result<(), &'static str> {
        if val < Self::from_starting_class(self.class).level {
            return Err("level cannot be lower than starting class level");
        }

        let lvl_cap = 713;
        if val > lvl_cap {
            return Err("level cannot be higher than cap, 713");
        }

        self.level = val;

        Ok(())
    }

    pub fn change_starter_class(&mut self, target_class: StartingClassType) {
        let target_stats = Self::from_starting_class(target_class);
        let current_starter_class_stats = Self::from_starting_class(self.class);

        for stat in CoreStat::iter_scalings() {
            let target_stat = target_stats[stat];
            let current_stat = self[stat];
            let current_starter_stat = current_starter_class_stats[stat];

            let diff = target_stat - current_starter_stat;
            if current_stat == current_starter_stat {
                self[stat] = target_stat;
            }

            self.level += diff;
        }

        self.class = target_class;
    }
}

impl Index<CoreStat> for StatList {
    type Output = i32;

    fn index(&self, index: CoreStat) -> &Self::Output {
        match index {
            CoreStat::Vig => &self.vigor,
            CoreStat::Mnd => &self.mind,
            CoreStat::End => &self.endurance,
            CoreStat::Str => &self.strength,
            CoreStat::Dex => &self.dexterity,
            CoreStat::Int => &self.intelligence,
            CoreStat::Fai => &self.faith,
            CoreStat::Arc => &self.arcane,
        }
    }
}

impl IndexMut<CoreStat> for StatList {
    fn index_mut(&mut self, index: CoreStat) -> &mut Self::Output {
        match index {
            CoreStat::Vig => &mut self.vigor,
            CoreStat::Mnd => &mut self.mind,
            CoreStat::End => &mut self.endurance,
            CoreStat::Str => &mut self.strength,
            CoreStat::Dex => &mut self.dexterity,
            CoreStat::Int => &mut self.intelligence,
            CoreStat::Fai => &mut self.faith,
            CoreStat::Arc => &mut self.arcane,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_unspent_levels() {
        let mut stats = dbg!(StatList::from_slice_with_class_check(
            [60, 15, 40, 80, 14, 15, 6, 9],
            160,
            StartingClassType::Prisoner,
        )
        .unwrap());

        assert_eq!(stats.get_unspent_levels().unwrap(), 0);
    }

    #[test]
    fn invalid_unspent_levels() {
        let mut stats = dbg!(StatList::from_slice_with_class_check(
            [50, 15, 30, 50, 14, 15, 8, 11],
            40,
            StartingClassType::Hero,
        )
        .unwrap());

        assert_eq!(
            stats.get_unspent_levels().err(),
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
        let stats = StatList::from_slice_with_class_check(
            [50, 15, 30, 50, 50, 50, 8, 11],
            40,
            StartingClassType::Hero,
        )
        .unwrap();
        let weapon = weapons::Weapon::from_data("Moonveil", 3).expect("failed to get weapon");
        assert!(stats.check_weapon_requirements(&weapon));
    }

    #[test]
    fn test_stat_list_index() {
        let stats =
            StatList::from_slice([50, 15, 30, 50, 50, 50, 6, 9], 40, StartingClassType::Hero);
        assert_eq!(stats[CoreStat::Vig], 50);
        assert_eq!(stats[CoreStat::Mnd], 15);
        assert_eq!(stats[CoreStat::End], 30);
        assert_eq!(stats[CoreStat::Str], 50);
        assert_eq!(stats[CoreStat::Dex], 50);
        assert_eq!(stats[CoreStat::Int], 50);
        assert_eq!(stats[CoreStat::Fai], 6);
        assert_eq!(stats[CoreStat::Arc], 9);
    }

    #[test]
    fn push_to_weapon_requirements_success() {
        let mut stats = stats::StatList::from_slice_with_class_check(
            [60, 15, 40, 11, 14, 14, 6, 9],
            150,
            StartingClassType::Prisoner,
        )
        .expect("failed to create stats");
        let weapon =
            weapons::Weapon::from_data("Ruins Greatsword", 5).expect("failed to create weapon");
        stats = stats::StatList::push_stats_to_weapon_requirement(stats, &weapon)
            .expect("Failed weapon requirements check");

        dbg!(stats);
    }

    #[test]
    fn push_to_weapon_requirements_failure() {
        let stats = stats::StatList::from_slice_with_class_check(
            [10, 10, 10, 10, 10, 10, 10, 10],
            2,
            StartingClassType::Wretch,
        )
        .expect("failed to create stats");
        let weapon =
            weapons::Weapon::from_data("Ruins Greatsword", 5).expect("failed to create weapon");
        assert_eq!(
            stats::StatList::push_stats_to_weapon_requirement(stats, &weapon).err(),
            Some("Not enough levels to meet weapon requirements")
        );
    }

    #[test]
    fn check_stats_match_weapon_requirement_success() {
        let stats = stats::StatList::from_slice(
            [10, 10, 10, 11, 10, 10, 10, 10],
            2,
            StartingClassType::Wretch,
        );

        assert!(stats.check_stats_match_starting_class().is_ok());
    }

    #[test]
    fn check_stats_match_weapon_requirement_failure() {
        let stats = stats::StatList::from_slice(
            [10, 10, 10, 9, 10, 10, 10, 10],
            2,
            StartingClassType::Wretch,
        );

        assert_eq!(
            stats.check_stats_match_starting_class().err(),
            Some("level of stats is beneath starter level. this cannot happen")
        );
    }

    #[test]
    fn check_stats_match_weapon_requirement_success_2() {
        let stats = stats::StatList::from_slice(
            [60, 15, 40, 11, 14, 14, 6, 9],
            150,
            StartingClassType::Prisoner,
        );

        assert!(stats.check_stats_match_starting_class().is_ok());
    }

    #[test]
    fn check_stats_match_weapon_requirement_failure_2() {
        let stats = stats::StatList::from_slice(
            [60, 15, 40, 11, 14, 14, 6, 9],
            5,
            StartingClassType::Prisoner,
        );

        assert_eq!(
            stats.check_stats_match_starting_class().err(),
            Some("level of stats is beneath starter level. this cannot happen")
        );
    }

    #[test]
    fn try_change_stat_failure() {
        let mut stats = stats::StatList::from_slice_with_class_check(
            [60, 15, 40, 11, 14, 14, 6, 9],
            150,
            StartingClassType::Prisoner,
        )
        .expect("failed to create stats");

        assert_eq!(
            stats.change_stat(CoreStat::Vig, 0).err(),
            Some("stat cannot be lower than starting class stat")
        );
    }

    #[test]
    fn try_change_stat_success() {
        let mut stats = stats::StatList::from_slice_with_class_check(
            [60, 15, 40, 11, 14, 14, 6, 9],
            150,
            StartingClassType::Prisoner,
        )
        .expect("failed to create stats");

        assert!(stats.change_stat(CoreStat::Vig, 30).is_ok());
    }

    #[test]
    fn try_change_starting_class() {
        let mut stats = stats::StatList::from_slice_with_class_check(
            [60, 15, 40, 11, 14, 14, 6, 9],
            150,
            StartingClassType::Prisoner,
        )
        .expect("failed to create stats");

        let expected_stats = stats::StatList::from_slice_with_class_check(
            [60, 15, 40, 8, 12, 16, 7, 9],
            148,
            StartingClassType::Astrologer,
        )
        .expect("failed to create stats");

        stats.change_starter_class(StartingClassType::Astrologer);
        dbg!(&stats);

        assert_eq!(stats, expected_stats);
    }

    #[test]
    fn try_change_starting_class_same_class() {
        let mut stats = stats::StatList::from_slice_with_class_check(
            [60, 15, 40, 11, 14, 14, 6, 9],
            150,
            StartingClassType::Prisoner,
        )
        .expect("failed to create stats");

        let expected_stats = stats::StatList::from_slice_with_class_check(
            [60, 15, 40, 11, 14, 14, 6, 9],
            150,
            StartingClassType::Prisoner,
        )
        .expect("failed to create stats");

        stats.change_starter_class(StartingClassType::Prisoner);
        dbg!(&stats);

        assert_eq!(stats, expected_stats);
    }

    #[test]
    fn change_level_valid() {
        let target_lvl = 45;
        let mut stats = stats::StatList::from_starting_class(StartingClassType::Wretch);

        let expected_stats = stats::StatList::from_slice_with_class_check(
            [10, 10, 10, 10, 10, 10, 10, 10],
            45,
            StartingClassType::Wretch,
        )
        .expect("failed to create stats");

        stats
            .change_level(target_lvl)
            .expect("failed to change level");
        dbg!(&stats);
        assert_eq!(stats, expected_stats);
    }

    #[test]
    fn change_level_below_min_invalid() {
        let target_lvl = 8;
        let mut stats = stats::StatList::from_starting_class(StartingClassType::Prisoner);
        assert_eq!(
            stats.change_level(target_lvl).err(),
            Some("level cannot be lower than starting class level")
        );
    }

    #[test]
    fn change_level_above_cap_invalid() {
        let target_lvl = 714;
        let mut stats = stats::StatList::from_starting_class(StartingClassType::Wretch);
        assert_eq!(
            stats.change_level(target_lvl).err(),
            Some("level cannot be higher than cap, 713")
        );
    }
}
