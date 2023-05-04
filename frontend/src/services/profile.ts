import axios from 'axios';

export function get_info(setProfileData: React.SetStateAction<any>) {
  axios({
    method: "GET", 
    url: "/profile",
  })
  .then((response) => {
    const res = response.data
    setProfileData(({
      profile_name: res.name,
      profile_about: res.about,
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
