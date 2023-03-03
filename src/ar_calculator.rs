// TODO: most query functions here need error handling in case of fail
use super::*;

/// query the csv to get the attack element parameters given id
pub fn get_attack_element_param(
    attack_element_correct_id: i32,
) -> Result<Vec<i32>, Box<dyn Error>> {
    let path = Path::new("csv_data/AttackElementCorrectParam.csv");
    let mut rdr = csv::Reader::from_path(path)?;

    let mut out: Vec<i32> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        if record.get(0).unwrap() == attack_element_correct_id.to_string() {
            // println!("{:?}", record);
            out = (1..record.len())
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
        return Err("failed to get attack element param".into());
    }

    Ok(out)
}

/// query reinforce param csv for param
pub fn get_reinforce_param_modifier(reinforce_param_id: i32) -> Result<Vec<f32>, Box<dyn Error>> {
    let path = Path::new("csv_data/ReinforceParamWeapon.csv");
    let mut rdr = csv::Reader::from_path(path)?;

    let mut out: Vec<f32> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        if record.get(0).unwrap() == reinforce_param_id.to_string() {
            // println!("{:?}", record);
            out = (1..12)
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

    Ok(out)
}

/// query the csv to get the calc correct graph
fn get_calc_correct_graph_ids(weapon_name: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let path = Path::new("csv_data/CalcCorrectGraphID.csv");
    let mut rdr = csv::Reader::from_path(path)?;

    let mut out: Vec<i32> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        if record.get(1).unwrap() == weapon_name {
            // println!("{:?}", record);
            out = (2..record.len())
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
        return Err("failed to get calc correct graph ids".into());
    }

    Ok(out)
}

/// query the calc correct info given the id
fn get_calc_correct_graphs(
    calc_correct_ids: &Vec<i32>,
) -> Result<HashMap<i32, Vec<f32>>, Box<dyn Error>> {
    let path = Path::new("csv_data/CalcCorrectGraph.csv");
    let mut rdr = csv::Reader::from_path(path)?;

    let mut calc_correct_graphs: HashMap<i32, Vec<f32>> = HashMap::new();

    for id in calc_correct_ids {
        if calc_correct_graphs.get(&id).is_none() {
            for result in rdr.records() {
                let record = result?;
                if record.get(0).unwrap() == id.to_string() {
                    // println!("{:?}", record);
                    let graph: Vec<f32> = (2..record.len())
                        .map(|i| {
                            record
                                .get(i)
                                .unwrap()
                                .parse()
                                .expect("failed to translate entry to number")
                        })
                        .collect();

                    calc_correct_graphs.insert(*id, graph);

                    break;
                }
            }

            if rdr.is_done() {
                return Err("failed to get calc correct graph".into());
            }
        }
    }

    Ok(calc_correct_graphs)
}

// change this to take graph as a 2d array where rows are stat, growth, exp and cols are values
fn calc_correct(input: i32, graph: &Vec<f32>) -> f32 {
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

/// calculate the type damage added to the AR based on the stat
fn dmg_type_per_stat(base_attack: f32, weapon_scaling: f32, calc_correct_result: f32) -> f32 {
    base_attack * (weapon_scaling / 100.0) * (calc_correct_result / 100.0)
}


// TODO this should maybe return an error in certain cases
/// return the weapon ar given the weapon and corresponding character statlist
pub fn calculate_ar(weapon: &weapon::Weapon, statlist: &StatList) -> f32 {
    let mut total_ar: f32 = 0.0;
    let attack_element_param = get_attack_element_param(weapon.attack_element_correct_id).unwrap();
    let calc_correct_ids = get_calc_correct_graph_ids(&weapon.name).unwrap();
    let calc_correct_graphs = get_calc_correct_graphs(&calc_correct_ids).unwrap();

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
            println!("\ncurrent stat: {}", current_attack_stat);
            total_ar += current_attack_stat;

            let calc_correct_graph: &Vec<f32> = calc_correct_graphs
                .get(&calc_correct_ids[i])
                .expect("failed to index calc_correct_ids");

            for j in 0..5 {
                let (weapon_scaling, stat) = match j {
                    0 => (weapon.get_scaling_stat(Scaling::Str), statlist.strength),
                    1 => (weapon.get_scaling_stat(Scaling::Dex), statlist.dexterity),
                    2 => (weapon.get_scaling_stat(Scaling::Int), statlist.intelligence),
                    3 => (weapon.get_scaling_stat(Scaling::Fai), statlist.faith),
                    4 => (weapon.get_scaling_stat(Scaling::Arc), statlist.arcane),
                    _ => (0.0, 0),
                };

                if attack_element_param[(5 * i) + j] == 1 {
                    let calc_correct_value = dbg!(calc_correct(stat, &calc_correct_graph));
                    let dmg =
                        dmg_type_per_stat(current_attack_stat, weapon_scaling, calc_correct_value);
                    println!("scaling {}: {}", (5 * i) + j, dmg);
                    total_ar += dmg;
                }
            }
        }
    }

    total_ar.floor()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::weapon::Weapon;

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
        };

        let cold_dagger_5 = Weapon::build_from_data("Cold Dagger", 5).unwrap();
        let cold_dagger_ar = calculate_ar(&cold_dagger_5, &stats);
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
        };

        let fire_flamberge_5 = Weapon::build_from_data("Fire Flamberge", 0).unwrap();
        let fire_flamberge_ar = calculate_ar(&fire_flamberge_5, &stats);
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
        };

        let ruins_gs_0 = Weapon::build_from_data("Ruins Greatsword", 0).unwrap();
        let ruins_gs_ar = calculate_ar(&ruins_gs_0, &stats);
        assert_eq!(ruins_gs_ar, 247.0);
    }

    #[test]
    fn invalid_name() {
        let ruins_gs_0 = Weapon::build_from_data("fiaonwe", 0);
        assert!(ruins_gs_0.is_err());
    }

    #[test]
    fn invalid_upgrade_level() {
        let ruins_gs_0 = Weapon::build_from_data("Ruins Greatsword", 12);
        assert!(ruins_gs_0.is_err());
    }

    #[test]
    fn weapon_modification_ruins_gs_5_to_10() {
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
        };

        let mut ruins_gs_5 = dbg!(Weapon::build_from_data("Ruins Greatsword", 5).unwrap());
        let ruins_gs_5_ar = calculate_ar(&ruins_gs_5, &stats);
        assert_eq!(ruins_gs_5_ar, 487.0);

        ruins_gs_5.upgrade_weapon(10);
        let ruins_gs_10_ar = calculate_ar(&ruins_gs_5, &stats);
        assert_eq!(ruins_gs_10_ar, 777.0);
    }
}
