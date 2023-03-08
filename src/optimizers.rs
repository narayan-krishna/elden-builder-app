use super::*;
use ar_calculator::csv_parsing;

/// optimize the given statlist for a given weapon within a given amoutn of level points and return it as a new statlist
pub fn optimize(
    weapon: &weapons::Weapon,
    statlist: &stats::StatList,
) -> Result<stats::StatList, Box<dyn Error>> {
    let mut optimized_statlist = meet_weapon_requirements(statlist.clone(), weapon)?;

    let unspent_levels = dbg!(optimized_statlist.unspent_levels()?);
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

    let mut allocation: Option<Scaling> = None;
    for _i in 0..unspent_levels {
        let mut local_max = max_ar;
        // dbg!(local_max);
        for scaling in &relevant_scaling_stats {
            let mut temp_statlist = optimized_statlist.clone();

            match scaling {
                Scaling::Str => temp_statlist.strength += 1,
                Scaling::Dex => temp_statlist.dexterity += 1,
                Scaling::Int => temp_statlist.intelligence += 1,
                Scaling::Fai => temp_statlist.faith += 1,
                Scaling::Arc => temp_statlist.arcane += 1,
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

            // eprintln!("The AR for stat {:?} is {}", scaling, new_ar);

            if new_ar > local_max {
                allocation = Some(*scaling);
                local_max = new_ar;
            }
        }

        eprintln!("Allocating a point into {:?}", allocation.unwrap());
        match allocation {
            Some(Scaling::Str) => optimized_statlist.strength += 1,
            Some(Scaling::Dex) => optimized_statlist.dexterity += 1,
            Some(Scaling::Int) => optimized_statlist.intelligence += 1,
            Some(Scaling::Fai) => optimized_statlist.faith += 1,
            Some(Scaling::Arc) => optimized_statlist.arcane += 1,
            None => {}
        }

        max_ar = local_max;
        allocation = None;
    }

    Ok(optimized_statlist)
}

/// update a stat list so that it meets weapon requirements
// TODO: better name
pub fn meet_weapon_requirements(
    mut stats: stats::StatList,
    weapon: &weapons::Weapon,
) -> Result<stats::StatList, &'static str> {
    let mut unspent = stats.unspent_levels()?;

    for stat_type in [
        Scaling::Str,
        Scaling::Dex,
        Scaling::Int,
        Scaling::Fai,
        Scaling::Arc,
    ] {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meet_weapon_requirements_success() {
        let mut stats = stats::StatList::from_slice(
            [60, 15, 40, 11, 14, 14, 6, 9],
            150,
            StartingClassType::Prisoner,
        )
        .expect("failed to create stats");
        let weapon =
            weapons::Weapon::from_data("Ruins Greatsword", 5).expect("failed to create weapon");
        stats = meet_weapon_requirements(stats, &weapon).expect("Failed weapon requirements check");

        dbg!(stats);
    }

    #[test]
    fn meet_weapon_requirements_failure() {
        let stats = stats::StatList::from_slice(
            [10, 10, 10, 10, 10, 10, 10, 10],
            2,
            StartingClassType::Wretch,
        )
        .expect("failed to create stats");
        let weapon =
            weapons::Weapon::from_data("Ruins Greatsword", 5).expect("failed to create weapon");
        assert_eq!(
            meet_weapon_requirements(stats, &weapon).err(),
            Some("Not enough levels to meet weapon requirements")
        );
    }

    #[test]
    fn ruins_gs_str_only_optimization() {
        let mut stats = stats::StatList::from_slice(
            [60, 15, 40, 11, 14, 14, 6, 9],
            150,
            StartingClassType::Prisoner,
        )
        .expect("failed to create stats");
        assert_eq!(stats.unspent_levels().unwrap(), 60);
        let weapon =
            weapons::Weapon::from_data("Ruins Greatsword", 5).expect("failed to create weapon");

        let optimized_stats = dbg!(optimize(&weapon, &stats));
        assert_eq!(
            optimized_stats.expect("failed to get optimized stats"),
            stats::StatList::from_slice(
                [60, 15, 40, 69, 14, 16, 6, 9],
                150,
                StartingClassType::Prisoner,
            )
            .expect("failed to create stats")
        );
    }

    #[test]
    fn advanced_fallingstar_beastjaw_optimization() {
        let mut stats = stats::StatList::from_slice(
            [60, 15, 40, 11, 14, 14, 6, 9],
            150,
            StartingClassType::Prisoner,
        )
        .expect("failed to create stats");
        assert_eq!(stats.unspent_levels().unwrap(), 60);
        let weapon =
            weapons::Weapon::from_data("Fallingstar Beast Jaw", 5).expect("failed to create weapon");

        assert!(dbg!(optimize(&weapon, &stats).is_ok()));
    }
}
