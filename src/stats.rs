use super::*;

#[derive(Debug)]
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
