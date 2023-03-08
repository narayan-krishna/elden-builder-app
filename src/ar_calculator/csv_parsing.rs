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

/// query the csv for modifiers depending on the upgrade
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
pub fn get_calc_correct_graph_ids(weapon_name: &str) -> Result<Vec<i32>, Box<dyn Error>> {
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
pub fn get_calc_correct_graphs(
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
