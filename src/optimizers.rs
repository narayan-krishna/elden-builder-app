use super::*;
use ar_calculator::csv_parsing;

/// optimize the given statlist for a given weapon within a given amoutn of level points and return it as a new statlist
pub fn optimize_statlist_for_weapon(
    weapon: &weapons::Weapon,
    statlist: &stats::StatList,
) -> Result<stats::StatList, Box<dyn Error>> {
    let mut optimized_statlist =
        stats::StatList::push_stats_to_weapon_requirement(statlist.clone(), weapon)?;

    let unspent_levels = dbg!(optimized_statlist.get_unspent_levels()?);
    let relevant_scaling_stats = weapon.get_non_zero_attack_stats();
    let attack_element_param =
        csv_parsing::get_attack_element_param(weapon.attack_element_correct_id)?;
    let calc_correct_ids = csv_parsing::get_calc_correct_graph_ids(&weapon.name)?;
    let calc_correct_graphs = csv_parsing::get_calc_correct_graphs(&calc_correct_ids)?;

    let mut max_ar: f32 = ar_calculator::calculate_ar_core(
        weapon,
        &optimized_statlist,
        &attack_element_param,
        &calc_correct_ids,
        &calc_correct_graphs,
        false,
    )?;

    let mut allocation: Option<CoreStat> = None;
    for _i in 0..unspent_levels {
        let mut local_max = max_ar;
        for scaling in &relevant_scaling_stats {
            let mut temp_statlist = optimized_statlist.clone();
            // temp_statlist[*scaling] += 1;

            if let Err(e) = temp_statlist.change_stat(*scaling, temp_statlist[*scaling] + 1) {
                eprintln!("{}", e);
            }

            let new_ar = ar_calculator::calculate_ar_core(
                weapon,
                &temp_statlist,
                &attack_element_param,
                &calc_correct_ids,
                &calc_correct_graphs,
                false,
            )
            .unwrap();

            if new_ar > local_max {
                allocation = Some(*scaling);
                local_max = new_ar;
            }
        }

        eprintln!("Allocating a point into {:?}", allocation.unwrap());
        if let Some(stat) = allocation {
            optimized_statlist[stat] += 1;
        } else {
            return Err("Allocation was never assigned, failed to allocate statpoint".into());
        }

        max_ar = local_max;
        allocation = None;
    }

    Ok(optimized_statlist)
}

/// find the optimal character choice for a particular weapon, at a particular level.
// if no level is assigned, this can default at 100/120/150
pub fn optimize_character_selection_for_weapon(weapon: weapons::Weapon, level: i32) {
    todo!()
    // how do we do this?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ruins_gs_str_only_optimization() {
        let mut stats = stats::StatList::from_slice_with_class_check(
            [60, 15, 40, 11, 14, 14, 6, 9],
            150,
            StartingClassType::Prisoner,
        )
        .expect("failed to create stats");
        assert_eq!(stats.get_unspent_levels().unwrap(), 60);
        let weapon =
            weapons::Weapon::from_data("Ruins Greatsword", 5).expect("failed to create weapon");

        let optimized_stats = dbg!(optimize_statlist_for_weapon(&weapon, &stats));
        assert_eq!(
            optimized_stats.expect("failed to get optimized stats"),
            stats::StatList::from_slice_with_class_check(
                [60, 15, 40, 69, 14, 16, 6, 9],
                150,
                StartingClassType::Prisoner,
            )
            .expect("failed to create stats")
        );
    }

    #[test]
    fn advanced_fallingstar_beastjaw_optimization() {
        let mut stats = stats::StatList::from_slice_with_class_check(
            [60, 15, 40, 11, 14, 14, 6, 9],
            150,
            StartingClassType::Prisoner,
        )
        .expect("failed to create stats");
        assert_eq!(stats.get_unspent_levels().unwrap(), 60);
        let weapon = weapons::Weapon::from_data("Fallingstar Beast Jaw", 5)
            .expect("failed to create weapon");

        assert!(dbg!(optimize_statlist_for_weapon(&weapon, &stats).is_ok()));
    }

    #[test]
    fn vykes_war_spear_wretch_optimization() {
        // ar -> 700
        // vs. 660
        let stats = stats::StatList::from_slice_with_class_check(
            [15, 20, 30, 14, 13, 9, 9, 7],
            108,
            StartingClassType::Vagabond,
        )
        .expect("failed to create stats");

        let weapon =
            weapons::Weapon::from_data("Vyke's War Spear", 10).expect("failed to creat weapon");

        dbg!(optimize_statlist_for_weapon(&weapon, &stats).unwrap());
    }
 
    #[test]
    fn chritty_greatsword_optimization() {
        let stats = stats::StatList::from_slice_with_class_check(
            [10, 10, 10, 10, 10, 10, 10, 10],
            40,
            StartingClassType::Wretch,
        )
        .expect("failed to create stats");
        let weapon = weapons::Weapon::from_data("Greatsword", 1).expect("failed to create weapon");

        dbg!(optimize_statlist_for_weapon(&weapon, &stats).unwrap());
    }
}
