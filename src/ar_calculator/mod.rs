pub mod csv_parsing;
use std::collections::HashMap;

use super::*;

pub fn calculate_ar(
    weapon: &weapons::Weapon,
    statlist: &stats::StatList,
) -> Result<f32, Box<dyn Error>> {
    let attack_element_param =
        csv_parsing::get_attack_element_param(weapon.attack_element_correct_id)?;
    let calc_correct_ids = csv_parsing::get_calc_correct_graph_ids(&weapon.name)?;
    let calc_correct_graphs = csv_parsing::get_calc_correct_graphs(&calc_correct_ids)?;

    calculate_ar_core(
        weapon,
        statlist,
        &attack_element_param,
        &calc_correct_ids,
        &calc_correct_graphs,
        true,
    )
}

/// core ar calculator given all necessary values (attack element parameter, calc correct ids, calc
/// correct graphs). used in operations where the ar needs to be calculated multiple times for the
/// same weapon.
pub fn calculate_ar_core(
    weapon: &weapons::Weapon,
    statlist: &stats::StatList,
    attack_element_param: &Vec<i32>,
    calc_correct_ids: &Vec<i32>,
    calc_correct_graphs: &HashMap<i32, Vec<f32>>,
    floor: bool,
) -> Result<f32, Box<dyn Error>> {
    // TODO: check that stats meet weapon reqs

    let mut total_ar: f32 = 0.0;

    for i in 0..5 {
        let current_attack_stat = match i {
            0 => weapon.get_attack_stat(Attack::Physical),
            1 => weapon.get_attack_stat(Attack::Magic),
            2 => weapon.get_attack_stat(Attack::Fire),
            3 => weapon.get_attack_stat(Attack::Lightning),
            4 => weapon.get_attack_stat(Attack::Holy),
            _ => 0.0,
        };

        if current_attack_stat > 0.0 {
            // println!("\ncurrent stat: {}", current_attack_stat);
            total_ar += current_attack_stat;

            let calc_correct_graph: &Vec<f32> = calc_correct_graphs
                .get(&calc_correct_ids[i])
                .expect("failed to index calc_correct_ids");

            for j in 0..5 {
                let (weapon_scaling, stat) = match j {
                    0 => (weapon.get_scaling_stat(CoreStat::Str), statlist.strength),
                    1 => (weapon.get_scaling_stat(CoreStat::Dex), statlist.dexterity),
                    2 => (
                        weapon.get_scaling_stat(CoreStat::Int),
                        statlist.intelligence,
                    ),
                    3 => (weapon.get_scaling_stat(CoreStat::Fai), statlist.faith),
                    4 => (weapon.get_scaling_stat(CoreStat::Arc), statlist.arcane),
                    _ => (0.0, 0),
                };

                if attack_element_param[(5 * i) + j] == 1 {
                    let calc_correct_value = ar_calculator::calc_correct(stat, &calc_correct_graph);
                    let dmg =
                        dmg_type_per_stat(current_attack_stat, weapon_scaling, calc_correct_value);
                    total_ar += dmg;
                }
            }
        }
    }

    match floor {
        true => Ok(total_ar.floor()),
        false => Ok(total_ar),
    }
}

/// calculate the type damage added to the AR based on the stat
fn dmg_type_per_stat(base_attack: f32, weapon_scaling: f32, calc_correct_result: f32) -> f32 {
    base_attack * (weapon_scaling / 100.0) * (calc_correct_result / 100.0)
}

// TODO: change this to take graph as a 2d array where rows are stat, growth, exp and cols are values
/// calculate the calc correct value for a certain  stat input
pub fn calc_correct(input: i32, graph: &Vec<f32>) -> f32 {
    if input == 0 {
        return 0.0;
    }

    // calculate stat min, stat max, exp min, exp max, growth min, growth max
    let mut get_max_index: Option<usize> = None;
    for i in 0..5 {
        if input < graph[i] as i32 {
            get_max_index = Some(i);
            break;
        }
    }

    let max_index = get_max_index.expect("max_index never assigned");
    let min_index = max_index - 1;

    let stat_min = graph[min_index];
    let stat_max = graph[max_index];
    let grow_min = graph[min_index + 5];
    let grow_max = graph[max_index + 5];
    let exp_min = graph[min_index + 10];
    let _exp_max = graph[max_index + 10];

    let ratio: f32 = (input as f32 - stat_min) / (stat_max - stat_min);
    let growth: f32;

    match exp_min > 0.0 {
        true => growth = f32::powf(ratio, exp_min),
        false => growth = 1.0 - f32::powf(1.0 - ratio, exp_min.abs()),
    }

    let output = grow_min + ((grow_max - grow_min) * growth);

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::StatList;
    use crate::weapons::Weapon;

    #[test]
    fn valid_get_attack_element_param() {
        let attack_element_param = csv_parsing::get_attack_element_param(10013).unwrap();
        assert_eq!(
            attack_element_param,
            vec![1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0]
        );
    }

    #[test]
    fn valid_get_reinforce_param_modifier() {
        let reinforce_param_modifier = csv_parsing::get_reinforce_param_modifier(402).unwrap();
        assert_eq!(
            reinforce_param_modifier,
            vec![0.872, 0.872, 0.872, 0.872, 0.872, 1.08, 1.38, 0.56, 1.08, 1.08, 1.08]
        )
    }

    #[test]
    fn valid_get_calc_correct_graph_ids() {
        let calc_correct_graph_ids =
            csv_parsing::get_calc_correct_graph_ids("Miquellan Knight's Sword").unwrap();
        assert_eq!(calc_correct_graph_ids, vec![0, 4, 0, 0, 4]);
    }

    #[test]
    fn valid_get_calc_correct_graphs() {
        let calc_correct_graph_ids = vec![0, 4, 0, 0, 4];
        let calc_correct_graph =
            csv_parsing::get_calc_correct_graphs(&calc_correct_graph_ids).unwrap();

        assert_eq!(
            calc_correct_graph.get(&0).unwrap(),
            &vec![
                1.0, 18.0, 60.0, 80.0, 150.0, 0.0, 25.0, 75.0, 90.0, 110.0, 1.2, -1.2, 1.0, 1.0,
                1.0
            ]
        );
        assert_eq!(
            calc_correct_graph.get(&4).unwrap(),
            &vec![
                1.0, 20.0, 50.0, 80.0, 99.0, 0.0, 40.0, 80.0, 95.0, 100.0, 1.0, 1.0, 1.0, 1.0, 1.0
            ]
        );
    }

    #[test]
    fn cold_dagger_5_ar() {
        let stats = StatList {
            level: 10,
            vigor: 10,
            mind: 10,
            endurance: 10,
            strength: 14,
            dexterity: 14,
            intelligence: 40,
            faith: 10,
            arcane: 10,
            class: StartingClassType::Hero,
        };

        let cold_dagger_5 = Weapon::from_data("Cold Dagger", 5).unwrap();
        let cold_dagger_ar = calculate_ar(&cold_dagger_5, &stats).unwrap();
        assert_eq!(cold_dagger_ar, 173.0);
    }

    #[test]
    fn fire_flamberge_0_ar() {
        let stats = StatList {
            level: 10,
            vigor: 10,
            mind: 10,
            endurance: 10,
            strength: 15,
            dexterity: 14,
            intelligence: 40,
            faith: 10,
            arcane: 10,
            class: StartingClassType::Hero,
        };

        let fire_flamberge_5 = Weapon::from_data("Fire Flamberge", 0).unwrap();
        let fire_flamberge_ar = calculate_ar(&fire_flamberge_5, &stats).unwrap();
        assert_eq!(fire_flamberge_ar, 224.0);
    }

    #[test]
    fn ruins_gs_0_ar() {
        let stats = StatList {
            level: 10,
            vigor: 10,
            mind: 10,
            endurance: 10,
            strength: 50,
            dexterity: 14,
            intelligence: 40,
            faith: 10,
            arcane: 10,
            class: StartingClassType::Hero,
        };

        let ruins_gs_0 = dbg!(Weapon::from_data("Ruins Greatsword", 0).unwrap());
        let ruins_gs_ar = calculate_ar(&ruins_gs_0, &stats).unwrap();
        assert_eq!(ruins_gs_ar, 247.0);
    }

    #[test]
    fn ruins_gs_5_wretch_stats() { 
        todo!()
    }
}
