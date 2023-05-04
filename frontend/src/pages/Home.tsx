import { useState } from 'react';
import StatList from '../components/StatList'
import Box from "@mui/material/Box";
import Stack from "@mui/material/Stack";
import WeaponBox from '../components/Weapon'
import Button from '@mui/material/Button'
import OptimizedList from '../components/OptimizedList'
import IconButton from '@mui/material/IconButton';
import SouthIcon from '@mui/icons-material/South';
import NorthIcon from '@mui/icons-material/North';
import './Home.css'
import { optimize } from '../services/stats'

export type StatListProps = {
  level: number;
  vigor: number;
  mind: number;
  endurance: number;
  strength: number;
  dexterity: number;
  intelligence: number;
  faith: number;
  arcane: number;
  class: string;
  min_level: number;
  min_vigor: number;
  min_mind: number;
  min_endurance: number;
  min_strength: number;
  min_dexterity: number;
  min_intelligence: number;
  min_faith: number;
  min_arcane: number;
};

export type OptimizeDataProps = {
  level: number;
  vigor: number;
  mind: number;
  endurance: number;
  strength: number;
  dexterity: number;
  intelligence: number;
  faith: number;
  arcane: number;
  classtype: number;
}

export type WeaponProps = {
  name: string;
  upgrade_lvl: number;
  max_upgrade_lvl: number;
}

export interface OptimizeListProps {
  statlist: StatListProps;
  optimizelist: OptimizeDataProps;
}

export interface WeaponFullProps {
    weaponprops: WeaponProps;
    setweaponprops: React.Dispatch<React.SetStateAction<WeaponProps>>
}

export interface StatFullProps {
  statListData: StatListProps;
  setStatListData: React.Dispatch<React.SetStateAction<StatListProps>>
}

function Home() {
  const [statListData, setStatListData] = useState({
    level: 150,
    vigor: 43,
    mind: 15,
    endurance: 40,
    strength: 11,
    dexterity: 17,
    intelligence: 18,
    faith: 6,
    arcane: 9,
    class: "Prisoner",
    min_level: 9,
    min_vigor: 11,
    min_mind: 12,
    min_endurance: 11,
    min_strength: 11,
    min_dexterity: 14,
    min_intelligence: 14,
    min_faith: 6,
    min_arcane: 9,
  })

  const [weaponData, setWeaponData] = useState({
    name: "Ruins Greatsword",
    upgrade_lvl: 5,
    max_upgrade_lvl: 10,
  })

  const [optimizedData, setOptimizedData] = useState(null)

  const page_state = {
    configuration: 0,
    result: 1,
  };

  const [pageState, setPageState] = useState({
    state: page_state.configuration
  })

  function handleScrollDown() {
    if (pageState.state == page_state.configuration) {
      setPageState({
        state: page_state.result
      })
      window.scroll({
        top: document.body.offsetHeight,
        left: 0, 
        behavior: 'smooth',
      });
    }
  }

  function handleScrollUp() {
    if (pageState.state == page_state.result) {
      setPageState({
        state: page_state.configuration
      })
      window.scroll({
        top: 0,
        left: 0, 
        behavior: 'smooth',
      });
    }
  }

  window.addEventListener('wheel', function(event: any) {
    // some logic
    event.preventDefault();
  }, {passive:false});

  // TODO: make this its own page (setup/configuration) and make weapon and stat state global
  return (
    <div className="App">

      <Box
        minHeight='100vh'
        display="flex"
        sx={{
        }}
        justifyContent="center"
        alignItems="center"
      >

        <Stack
            direction="column"
            spacing={2}
          >
          <div style={{display: 'flex', justifyContent:'center'}}>
            <StatList statListData={statListData} setStatListData={setStatListData}/>
          </div>
          <div style={{display: 'flex', justifyContent:'center'}}>
            <WeaponBox weaponprops={weaponData} setweaponprops={setWeaponData}/>
          </div>
          <Box
            display='flex'
            sx={{
              p: 1,
            }}
            justifyContent="center"
            alignItems="center"
          >
            <Button onClick={() => {optimize(statListData, weaponData, setOptimizedData); handleScrollDown()}} variant="contained">Optimize</Button>
          </Box>
        </Stack>

      </Box>

      <Box
        minHeight='100vh'
        display="flex"
        sx={{
          bgcolor: 'grey.700',
        }}
        justifyContent="center"
        alignItems="center"
      >
        {optimizedData &&
          <OptimizedList statlist={statListData} optimizelist={optimizedData}/>
        }
      </Box>

      {optimizedData && pageState.state == page_state.configuration && <div className="mybutton">
        <IconButton aria-label="south" onClick={() => {handleScrollDown()}}>
          <SouthIcon />
        </IconButton>
      </div>}

      {pageState.state == page_state.result && <div className="mybutton">
        <IconButton aria-label="north" onClick={() => {handleScrollUp()}}>
          <NorthIcon />
        </IconButton>
      </div>}

    </div>
  );
}

export default Home;
