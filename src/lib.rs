#![allow(dead_code)]

use axum::response::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;
use std::fmt::Display;
use std::ops::{Index, IndexMut};

mod ar_calculator;
mod optimizers;
mod stats;
mod weapons;

pub async fn get_profile() -> Json<Value> {
    Json(json!({"name": "krishna", "about": "loves to eat"}))
}

pub async fn get_statlist() -> Json<stats::StatList> {
    eprintln!("called backend statlist acquisition");
    let statlist = stats::StatList::from_slice_with_class_check(
        [60, 15, 40, 11, 17, 18, 6, 9],
        150,
        StartingClassType::Prisoner,
    )
    .expect("failed to create stats");

    Json(statlist)
}

pub async fn get_optimized_statlist(
    Json(payload): Json<OptimizationPayload>,
) -> Json<stats::StatList> {
    //TODO: this needs to take a weapon in addition to stats. for now we'll create a weapon here
    eprintln!("called backend statlist optimization");
    let weapon = weapons::Weapon::from_data(&payload.weapon_name, payload.upgrade_lvl).unwrap();
    let statlist =
        optimizers::optimize_statlist_for_weapon(&weapon, &payload.current_stats).unwrap();

    Json(statlist)
}

pub async fn get_reset_statlist(
    Json(payload): Json<UserTargetStartingClass>,
) -> Json<stats::StatList> {
    // repurpose to be this to be more generalized getting starting class stats
    eprintln!("called backend statlist reset, changed_reset");
    let statlist = stats::StatList::from_starting_class(payload.target_starting_class);

    Json(statlist)
}

// TODO: change this to return FullStatList or not?
pub async fn change_starter_class(
    Json(mut payload): Json<UserChangeStartingClass>,
) -> Json<FullStatlist> {
    eprintln!("called backend starter class change");
    payload
        .current_stats
        .change_starter_class(payload.target_starting_class);

    let full_stat_list = FullStatlist {
        current_stats: payload.current_stats,
        min_stats: stats::StatList::from_starting_class(payload.target_starting_class),
    };

    Json(full_stat_list)
}

pub async fn provide_weapon_data(Json(payload): Json<WeaponName>) -> Json<weapons::Weapon> {
    eprintln!("called backend get weapon data");
    let wpn = weapons::Weapon::from_data(&payload.name, 1).expect("failed to create weapon");

    Json(wpn)
}

#[derive(Serialize, Deserialize)]
pub struct WeaponName {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserChangeStartingClass {
    // these should probably be more general purpose
    target_starting_class: StartingClassType,
    current_stats: stats::StatList,
}

#[derive(Serialize, Deserialize)]
pub struct UserTargetStartingClass {
    target_starting_class: StartingClassType,
}

#[derive(Serialize, Deserialize)]
pub struct FullStatlist {
    current_stats: stats::StatList,
    min_stats: stats::StatList,
}

#[derive(Serialize, Deserialize)]
pub struct OptimizationPayload {
    weapon_name: String,
    upgrade_lvl: i32,
    current_stats: stats::StatList,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CoreStat {
    Vig,
    Mnd,
    End,
    Str,
    Dex,
    Int,
    Fai,
    Arc,
}

impl CoreStat {
    fn iter_all() -> impl Iterator<Item = CoreStat> {
        [
            CoreStat::Vig,
            CoreStat::Mnd,
            CoreStat::End,
            CoreStat::Str,
            CoreStat::Dex,
            CoreStat::Int,
            CoreStat::Fai,
            CoreStat::Arc,
        ]
        .iter()
        .copied()
    }

    fn iter_scalings() -> impl Iterator<Item = CoreStat> {
        [
            CoreStat::Str,
            CoreStat::Dex,
            CoreStat::Int,
            CoreStat::Fai,
            CoreStat::Arc,
        ]
        .iter()
        .copied()
    }
}

pub enum Attack {
    Physical,
    Magic,
    Fire,
    Lightning,
    Holy,
    Stamina,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum StartingClassType {
    Hero,
    Bandit,
    Astrologer,
    Warrior,
    Prisoner,
    Confessor,
    Wretch,
    Vagabond,
    Prophet,
    Samurai,
}

type Levels = i32;
