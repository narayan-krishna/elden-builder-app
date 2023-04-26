import axios from 'axios';

// this needs the new min stats
export function change_starter_class(statListData, setStatListData, target_class) {
  axios({
    method: "POST",
    url: "/change_starter_class",
    data: {
      target_starting_class: target_class,
      current_stats: {
        level: statListData.level,
        vigor: statListData.vigor,
        mind: statListData.mind,
        endurance: statListData.endurance,
        strength: statListData.strength,
        dexterity: statListData.dexterity,
        intelligence: statListData.intelligence,
        faith: statListData.faith,
        arcane: statListData.arcane,
        class: statListData.class,
      }
    },
  })
  .then((response) => {
    const res = response.data
    setStatListData(({
      level: res.current_stats.level,
      vigor: res.current_stats.vigor,
      mind: res.current_stats.mind,
      endurance: res.current_stats.endurance,
      strength: res.current_stats.strength,
      dexterity: res.current_stats.dexterity,
      intelligence: res.current_stats.intelligence,
      faith: res.current_stats.faith,
      arcane: res.current_stats.arcane,
      class: res.current_stats.class,
      min_level: res.min_stats.level,
      min_vigor: res.min_stats.vigor,
      min_mind: res.min_stats.mind,
      min_endurance: res.min_stats.endurance,
      min_strength: res.min_stats.strength,
      min_dexterity: res.min_stats.dexterity,
      min_intelligence: res.min_stats.intelligence,
      min_faith: res.min_stats.faith,
      min_arcane: res.min_stats.arcane,
    }))
  })
  .catch((error) => {
    if (error.response) {
      console.log(error.response)
      console.log(error.response.status)
      console.log(error.response.headers)
    }
  })

}

// probably not needs, can simply reset to mins
export function reset(statListData, setStatListData) {
  console.log("resetting")
  axios({
    method: "POST",
    url: "/reset",
    data: {
      target_starting_class: statListData.class,
    }
  })
  .then((response) => {
    const res = response.data
    setStatListData(({
      level: res.level,
      vigor: res.vigor,
      mind: res.mind,
      endurance: res.endurance,
      strength: res.strength,
      dexterity: res.dexterity,
      intelligence: res.intelligence,
      faith: res.faith,
      arcane: res.arcane,
      class: res.class,
    }))
  })
  .catch((error) => {
    if (error.response) {
      console.log(error.response)
      console.log(error.response.status)
      console.log(error.response.headers)
    }
  })
}

// needs to use props
export function optimize(statListData, setStatListData, weaponData, setWeaponData) {
  axios({
    method: "POST",
    url: "/optimize",
    data: {
      level: statListData.level,
      vigor: statListData.vigor,
      mind: statListData.mind,
      endurance: statListData.endurance,
      strength: statListData.strength,
      dexterity: statListData.dexterity,
      intelligence: statListData.intelligence,
      faith: statListData.faith,
      arcane: statListData.arcane,
      class: statListData.class,
    }
  })
  .then((response) => {
    const res = response.data
    setStatListData(({
      level: res.level,
      vigor: res.vigor,
      mind: res.mind,
      endurance: res.endurance,
      strength: res.strength,
      dexterity: res.dexterity,
      intelligence: res.intelligence,
      faith: res.faith,
      arcane: res.arcane,
      class: res.class,
    }))
  })
  .catch((error) => {
    if (error.response) {
      console.log(error.response)
      console.log(error.response.status)
      console.log(error.response.headers)
    }
  })
}

// this needs new min stats
export function get_stats(setStatListData) {
  console.log("called get stats")
  axios({
    method: "GET",
    url: "/stats",
  })
  .then((response) => {
    const res = response.data
    console.log(res.level)
    setStatListData(({
      level: res.level,
      vigor: res.vigor,
      mind: res.mind,
      endurance: res.endurance,
      strength: res.strength,
      dexterity: res.dexterity,
      intelligence: res.intelligence,
      faith: res.faith,
      arcane: res.arcane,
      class: res.class,
    }))
  })
  .catch((error) => {
    if (error.response) {
      console.log(error.response)
      console.log(error.response.status)
      console.log(error.response.headers)
    }
  })
}
