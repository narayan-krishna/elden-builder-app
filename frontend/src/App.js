import logo from './logo.svg';
import { useState } from 'react';
import './App.css';
import axios from 'axios';
import StatList from './StatList'

// axios:
  // method
  // then
  // catch

function App() {

  const [profileData, setProfileData] = useState(null)
  const [statListData, setStatListData] = useState(null)
  const [optimizerStatListData, setOptimizerStatListaData] = useState(null)
  const [weaponData, setWeapnData] = useState(null)

  function getInfo() {
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

  function getStats() {
    axios({
      method: "GET",
      url: "/stats",
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

  function optimizeStats() {
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

  function reset() {
    console.log("resetting")
    axios({
      method: "POST",
      url: "/reset",
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

  function change_starter_class() {
    axios({
      method: "POST",
      url: "/change_starter_class",
      data: {
        target_starting_class: "Prisoner",
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

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>


        <p>Acquire profile details: </p><button onClick={getInfo}>click here</button>
        {profileData && <div>
          <p> Profile name: {profileData.profile_name}</p>
          <p> Profile about: {profileData.profile_about}</p>
          </div>
        }

        <button onClick={statListData && optimizeStats}>optimize stats</button>
        <button onClick={getStats}>acquire base stats</button>
        <button onClick={reset}>reset build</button>
        <button onClick={change_starter_class}>change starter class</button>
        <StatList statListData={statListData}/>
      </header>
    </div>
  );
}

export default App;
