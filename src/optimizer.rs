use super::*;

/// optimize the given statlist for a given weapon within a given amoutn of level points and return it as a new statlist
pub fn optimize(
    weapon: &weapon::Weapon,
    statlist: &stats::StatList,
) -> Result<stats::StatList, Box<dyn Error>> {
    let mut optimized_statlist = statlist.clone();
    let unspent_levels = optimized_statlist.unspent_levels()?;
    // calculate the amount of levels to spend (level - sum(stats))

    // find the relevant stats that actually affect weapon ar (stat list stats which are not 0)
    // loop levels
    //      max ar
    //      loop relevant stats
    //          find stat boost would most greatly affect current max ar
    //
    Ok(optimized_statlist)
}
