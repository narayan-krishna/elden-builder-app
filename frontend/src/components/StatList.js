import Stack from "@mui/material/Stack";
import Box from "@mui/material/Box";
import Grid from "@mui/material/Grid";
import FormControl from "@mui/material/FormControl";
import InputLabel from "@mui/material/InputLabel";
import Select from "@mui/material/Select";
import MenuItem from "@mui/material/MenuItem";
import TextField from "@mui/material/TextField";
import { change_starter_class, reset, get_stats, optimize } from '../services/stats'
import { useState } from 'react';
import Button from '@mui/material/Button'

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
// this should be called the change class function everytime
function ClassSelector(props) {
  const handleClassChange = (event) => {
    change_starter_class(props.statListData, props.setStatListData, event.target.value)
  }

  return (
    <FormControl sx={{ m: 1, minWidth: 350 }} size="small">
      <InputLabel id="demo-select-small-label">Class</InputLabel>
      <Select
        labelId="demo-select-small-label"
        id="demo-select-small"
        value={props.statListData.class}
        label="Class"
        onChange={handleClassChange}
        style={{ height: "flex" }}
      >
        <MenuItem value={"Prisoner"}>Prisoner</MenuItem>
        <MenuItem value={"Warrior"}>Warrior</MenuItem>
        <MenuItem value={"Astrologer"}>Astrologer</MenuItem>
        <MenuItem value={"Vagabond"}>Vagabond</MenuItem>
      </Select>
    </FormControl>
  );
}

// incrementer field for a stat
// this field, if left blank, should be reset to a previously valid
// if the value is changed, on leave, it should be verified that the new value is actually valid
function IncrementerField(props) {
  const handleLeave = (event) => {
    // if the field is left blank, or is below the minimum or above max
    if (event.target.value === "" || parseInt(event.target.value) < props.min || parseInt(event.target.value) > 99) {
      console.log("invalid stat, reverted to:", props.target_stat)
      // spawn an error here for the user
      event.target.value = props.target_stat
    }

    // needs to be generic
    switch (props.stat) {
      case Stats.vigor:
        props.setStatListData(currVal => ({
          ...currVal,
          vigor: parseInt(event.target.value)
        }))
        // validify statlist
      case Stats.mind:
        props.setStatListData(currVal => ({
          ...currVal,
          mind: parseInt(event.target.value)
        }))
      case Stats.endurance:
        props.setStatListData(currVal => ({
          ...currVal,
          endurance: parseInt(event.target.value)
        }))
    }
  }

  return (
    <TextField
      id="outlined-number"
      InputLabelProps={{
        shrink: true,
      }}
      onBlur={handleLeave}
      defaultValue={props.target_stat} // needs to be generic
      onInput={(e) => {
          e.target.value = e.target.value.replace(/[^0-9]/g, "")
      }}
      inputProps={{ inputMode: 'numeric', pattern: '[0-9]*', style: { height: "17px" }}}
    />
  );
}

function LevelIncrementer(props) {
  // this should be a combination of a text field and a selector
  return (
    <Box
      sx={{
        height: 50,
        width: 350,
        backgroundColor: 'primary.dark',
        '&:hover': {
          backgroundColor: 'primary.main',
          opacity: [0.9, 0.8, 0.7],
        },
      }}
    >
      <Grid container spacing={2} alignItems="center">
        <Grid item xs={8}>
          <Box display="flex" justifyContent="left">
            <div>
              {props.stat}
            </div>
          </Box>
        </Grid>
        <Grid item xs={4}>
          <IncrementerField
            stat={props.stat}
            target_stat={props.target_stat}
            key={props.target_stat} // needs to be generic
            setStatListData={props.setStatListData}
            statListData={props.statListData}
            min={1}
          />
        </Grid>
      </Grid>
    </Box>
  );
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
      <div>
        <p> level: {statListData.level}, {statListData.min_level}</p>
        <p> vigor: {statListData.vigor}, {statListData.min_vigor}</p>
        <p> mind: {statListData.mind}, {statListData.min_mind}</p>
        <p> endurance: {statListData.endurance}, {statListData.min_endurance}</p>
        <p> strength: {statListData.strength}, {statListData.min_strength}</p>
        <p> dexterity: {statListData.dexterity}, {statListData.min_dexterity}</p>
        <p> intelligence: {statListData.intelligence}, {statListData.min_intelligence}</p>
        <p> faith: {statListData.faith}, {statListData.min_faith}</p>
        <p> arcane: {statListData.arcane}, {statListData.min_arcane}</p>
        <p> class: {statListData.class}</p>
      </div>
      <Box
        sx={{
          width: 400,
          height: 600,
          backgroundColor: 'grey',
        }}
      >
        <Stack spacing={{ xs: .1, sm: .4 }} direction="column" useFlexGap flexWrap="wrap" alignItems="center">
          {/*has individual size components*/}
          <ClassSelector 
            statListData={statListData} 
            setStatListData={setStatListData}
          />
          {/*has individual size components*/}
          <LevelIncrementer 
            stat={Stats.vigor}
            target_stat={statListData.vigor}
            setStatListData={setStatListData}
            statListData={statListData} 
          />
          <LevelIncrementer 
            stat={Stats.mind}
            target_stat={statListData.mind}
            setStatListData={setStatListData}
            statListData={statListData} 
          />
          <LevelIncrementer 
            stat={Stats.endurance}
            target_stat={statListData.endurance}
            setStatListData={setStatListData}
            statListData={statListData} 
          />
          <div>
            <Button onClick={() => get_stats(setStatListData)} variant="contained">Acquire Stats</Button>
            <Button onClick={() => {statListData && optimize(statListData, setStatListData)}} variant="contained">Optimize</Button>
            <Button onClick={() => {statListData && reset(statListData, setStatListData)}} variant="outlined" color="error">Reset</Button>
          </div>
        </Stack>
        <Grid container spacing={1}>
        </Grid>
      </Box>
    </div>
  );
}

export {StatList as default}
