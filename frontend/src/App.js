import logo from './logo.svg';
import { useState } from 'react';
import './App.css';
import StatList from './components/StatList'
import { get_info } from './services/profile'

// axios:
  // method
  // then
  // catch

function App() {

  const [profileData, setProfileData] = useState(null)

  const [optimizerStatListData, setOptimizerStatListaData] = useState(null)
  const [weaponData, setWeaponData] = useState(null)

  return (
    <div className="App">
      <p>Acquire profile details: </p><button onClick={() => get_info(setProfileData)}>click here</button>
      {profileData && <div>
        <p> Profile name: {profileData.profile_name}</p>
        <p> Profile about: {profileData.profile_about}</p>
        </div>
      }
      <div style={{display: 'flex', justifyContent:'center'}}>
        <StatList />
      </div>

    </div>
  );
}

export default App;
