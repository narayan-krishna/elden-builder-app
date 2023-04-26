import Stack from "@mui/material/Stack";
import Box from "@mui/material/Box";
import Divider from '@mui/material/Divider';
import FormControl from "@mui/material/FormControl";
import InputLabel from "@mui/material/InputLabel";
import Select from "@mui/material/Select";
import MenuItem from "@mui/material/MenuItem";
import TextField from "@mui/material/TextField";
import { change_starter_class, reset, get_stats, optimize } from '../services/stats'
import Typography from '@mui/material/Typography';
import { useState } from 'react';
import Button from '@mui/material/Button'
import commmonStyles from './common'

const commonStyles = {
  bgcolor: 'background.paper',
  borderColor: 'grey.500',
  border: 1,
  borderRadius: 1,
  // width: '5rem',
  // height: '5rem',
};

const Stats = {
  vigor: "Vigor",
  mind: "Mind",
  endurance: "Endurance",
  strength: "Strength",
  dexterity: "Dexterity",
  intelligence: "Intelligence",
  faith: "Faith",
  arcane: "Arcane",
}
// TODO: this should be called by the change class function everytime
function ClassSelector(props) {
  const handleClassChange = (event) => {
    change_starter_class(props.statListData, props.setStatListData, event.target.value)
  }

  return (
    <FormControl sx={{ m: 1, minWidth: 350, }} size="small">
      <InputLabel id="demo-select-small-label">Class</InputLabel>
      <Select
        labelId="demo-select-small-label"
        id="demo-select-small"
        value={props.statListData.class}
        label="Class"
        onChange={handleClassChange}
        style={{ height: "55px" }}
      >
        <MenuItem value={"Prisoner"}>Prisoner</MenuItem>
        <MenuItem value={"Warrior"}>Warrior</MenuItem>
        <MenuItem value={"Astrologer"}>Astrologer</MenuItem>
        <MenuItem value={"Vagabond"}>Vagabond</MenuItem>
      </Select>
    </FormControl>
  );
}

// TODO: implement a local reset function
// function reset() {
//
// }

// this should be more general purpose
function IncrementerField(props) {
  const handleLeave = (event) => {
    // if the field is left blank, or is below the minimum or above max
    if (event.target.value === "" || parseInt(event.target.value) < props.min || parseInt(event.target.value) > props.max) {
      console.log("invalid stat, reverted to:", props.target_stat)
      // TODO: spawn an error here for the user
      event.target.value = props.target_stat
    }

    console.log(props.stat)
    switch (props.stat) {
      case Stats.vigor:
        console.log("select vigor")
        props.setStatListData(currVal => ({
          ...currVal,
          vigor: parseInt(event.target.value)
        }))
        break;
      case Stats.mind:
        console.log("select mind")
        props.setStatListData(currVal => ({
          ...currVal,
          mind: parseInt(event.target.value)
        }))
        break;
      case Stats.endurance:
        console.log("select endurance")
        props.setStatListData(currVal => ({
          ...currVal,
          endurance: parseInt(event.target.value)
        }))
        break;
      case Stats.strength:
        console.log("select strength")
        props.setStatListData(currVal => ({
          ...currVal,
          strength: parseInt(event.target.value)
        }))
        break;
      case Stats.dexterity:
        console.log("select dexterity")
        props.setStatListData(currVal => ({
          ...currVal,
          dexterity: parseInt(event.target.value)
        }))
        break;
      case Stats.intelligence:
        console.log("select intelligence")
        props.setStatListData(currVal => ({
          ...currVal,
          intelligence: parseInt(event.target.value)
        }))
        break;
      case Stats.faith:
        console.log("select faith")
        props.setStatListData(currVal => ({
          ...currVal,
          faith: parseInt(event.target.value)
        }))
        break;
      case Stats.arcane:
        console.log("select arcane")
        props.setStatListData(currVal => ({
          ...currVal,
          arcane: parseInt(event.target.value)
        }))
        break;
    }
  }

  return (
    <TextField
      id="outlined-number"
      InputLabelProps={{
        shrink: true,
      }}
      onBlur={handleLeave}
      defaultValue={props.target_stat} // TODO: needs to be generic
      onInput={(e) => {
          e.target.value = e.target.value.replace(/[^0-9]/g, "")
      }}
      inputProps={{ inputMode: 'numeric', pattern: '[0-9]*', style: { height: "17px", width: "46px" }}}
    />
  );
}

function LevelIncrementer(props) {
  // TODO: this should be a combination of a text field and a selector
  return (
    <Box
      sx={{
        width: 350,
        height: 50,
        p: .5,
      }}
      justifyContent="center"
      alignItems="center"
    >
      <Stack direction="row" spacing={2}>
        <Box display="flex" justifyContent="left" alignItems="flex-end" sx={{flexGrow: 1, p: 0, }}>
          <div>
            <Typography variant="body1">{props.stat}</Typography>
          </div>
        </Box>
        <IncrementerField
          stat={props.stat}
          target_stat={props.target_stat}
          key={props.target_stat} // TODO: needs to be generic
          setStatListData={props.setStatListData}
          statListData={props.statListData}
          min={props.min}
          max={props.max}
        />
      </Stack>
    </Box>
  );
}

LevelIncrementer.defaultProps = {
  min: 0,
  max: 99,
}

// statlist component
// users should be able to change the props entered here. BUT they should not go beyond starter class
function StatList(props) {
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

  return (
    statListData && <div>
      <Box
        sx={{
          // width: 400,
          // height: 600,
          display: 'flex',
          justifyContent: 'center',
          alignItems: 'center',
          m: 1,
          p: 1,
          // backgroundColor: 'grey',
          ...commonStyles,
        }}
        border={0}
        borderLeft={1}
        borderRight={1}
      >
        <Stack spacing={{ xs: .1, sm: .4 }} direction="column" useFlexGap flexWrap="wrap" alignItems="center" justifyContent="center">
          {/*has individual size components*/}
          <ClassSelector 
            statListData={statListData} 
            setStatListData={setStatListData}
          />
          {/*has individual size components*/}
          <LevelIncrementer 
            stat={Stats.vigor}
            target_stat={statListData.vigor}
            min={statListData.min_vigor}
            setStatListData={setStatListData}
            statListData={statListData} 
          />
          <LevelIncrementer 
            stat={Stats.mind}
            target_stat={statListData.mind}
            min={statListData.min_mind}
            setStatListData={setStatListData}
            statListData={statListData} 
          />
          <LevelIncrementer 
            stat={Stats.endurance}
            target_stat={statListData.endurance}
            min={statListData.min_endurance}
            setStatListData={setStatListData}
            statListData={statListData} 
          />
          <LevelIncrementer 
            stat={Stats.strength}
            target_stat={statListData.strength}
            min={statListData.min_strength}
            setStatListData={setStatListData}
            statListData={statListData} 
          />
          <LevelIncrementer 
            stat={Stats.dexterity}
            target_stat={statListData.dexterity}
            min={statListData.min_dexterity}
            setStatListData={setStatListData}
            statListData={statListData} 
          />
          <LevelIncrementer 
            stat={Stats.intelligence}
            target_stat={statListData.intelligence}
            min={statListData.min_intelligence}
            setStatListData={setStatListData}
            statListData={statListData} 
          />
          <LevelIncrementer 
            stat={Stats.faith}
            target_stat={statListData.faith}
            min={statListData.min_faith}
            setStatListData={setStatListData}
            statListData={statListData} 
          />
          <LevelIncrementer 
            stat={Stats.arcane}
            target_stat={statListData.arcane}
            min={statListData.min_arcane}
            setStatListData={setStatListData}
            statListData={statListData} 
          />
          <Box
            sx={{
              m: 2,
            }}
          >
            <Stack
                direction="row"
                divider={<Divider orientation="vertical" flexItem />}
                spacing={2}
              >
              {/*<Button onClick={() => get_stats(setStatListData)} variant="contained">Acquire Stats</Button>*/}
              {/*<Button onClick={() => {statListData && optimize(statListData, setStatListData)}} variant="contained">Optim</Button>*/}
              <Button onClick={() => {statListData && reset(statListData, setStatListData)}} variant="outlined" color="error">Reset</Button>
            </Stack>
          </Box>
        </Stack>
      </Box>
    </div>
  );
}

export {StatList as default}
