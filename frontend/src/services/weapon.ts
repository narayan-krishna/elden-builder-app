import axios from 'axios';
import { WeaponProps } from '../pages/Home';

export function get_rand_weapon(setWeaponData: React.Dispatch<React.SetStateAction<WeaponProps>>) {
  console.log("called get weapon data")
  axios({
    method: "GET",
    // TODO: url for this function
    url: "/stats",
  })
  .then((response) => {
    const res = response.data
    console.log(res.level)
    setWeaponData(({
      name: res.name,
      upgrade_lvl: res.upgrade_lvl,
      max_upgrade_lvl: res.max_upgrade_lvl,
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

//BUG: cannot get rid of weaponData or breakage
export default function get_weapon_data(weaponData: WeaponProps, setWeaponData: React.Dispatch<React.SetStateAction<WeaponProps>>, weapon_name: string) {
  console.log("called get weapon data using name", weapon_name)
  axios({
    method: "POST",
    url: "/weapon_data",
    data: {
      name: weapon_name,
    }
  })
  .then((response) => {
    const res = response.data
    setWeaponData(({
      name: res.name,
      upgrade_lvl: res.upgrade_lvl,
      max_upgrade_lvl: res.max_upgrade_lvl,
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
