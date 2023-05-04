import Stack from "@mui/material/Stack";
import Box from "@mui/material/Box";
import Divider from '@mui/material/Divider';
import FormControl from "@mui/material/FormControl";
import InputLabel from "@mui/material/InputLabel";
import Select from "@mui/material/Select";
import MenuItem from "@mui/material/MenuItem";
import TextField from "@mui/material/TextField";
import { change_starter_class, reset, } from '../services/stats'
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button'
import { StatFullProps } from '../pages/Home';

const commonStyles = {
  bgcolor: 'background.paper',
  borderColor: 'grey.500',
  border: 1,
  borderRadius: 1,
  // width: '5rem',
  // height: '5rem',
};

export const Stats = {
  level: "Level",
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
const ClassSelector: React.FC<StatFullProps> = ({ statListData, setStatListData }) => {
  const handleClassChange = (event: any) => {
    change_starter_class(statListData, setStatListData, event.target.value)
  }

  return (
    <FormControl sx={{ m: 1, minWidth: 350, }} size="small">
      <InputLabel id="demo-select-small-label">Class</InputLabel>
      <Select
        labelId="demo-select-small-label"
        id="demo-select-small"
        value={statListData.class}
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

interface IncrementerProps {
  target_stat: number;
  stat: string;
  min: number;
  max: number;
  key: React.Key;
  statListProps: StatFullProps;
}

// this should be more general purpose
const IncrementerField: React.FC<IncrementerProps> = ({ min, max, target_stat, stat, statListProps }) => {
  const handleLeave = (event: any) => {
    // if the field is left blank, or is below the minimum or above max
    if (event.target.value === "" || 
        parseInt(event.target.value) < min || 
        parseInt(event.target.value) > max) {
      console.log("invalid stat, reverted to:", target_stat)
      // TODO: spawn an error here for the user
      event.target.value = target_stat
    }

    console.log(stat)
    switch (stat) {
      case Stats.level:
        console.log("select level")
        statListProps.setStatListData(currVal => ({
          ...currVal,
          level: parseInt(event.target.value)
        }))
        break;
      case Stats.vigor:
        console.log("select vigor")
        statListProps.setStatListData(currVal => ({
          ...currVal,
          vigor: parseInt(event.target.value)
        }))
        break;
      case Stats.mind:
        console.log("select mind")
        statListProps.setStatListData(currVal => ({
          ...currVal,
          mind: parseInt(event.target.value)
        }))
        break;
      case Stats.endurance:
        console.log("select endurance")
        statListProps.setStatListData(currVal => ({
          ...currVal,
          endurance: parseInt(event.target.value)
        }))
        break;
      case Stats.strength:
        console.log("select strength")
        statListProps.setStatListData(currVal => ({
          ...currVal,
          strength: parseInt(event.target.value)
        }))
        break;
      case Stats.dexterity:
        console.log("select dexterity")
        statListProps.setStatListData(currVal => ({
          ...currVal,
          dexterity: parseInt(event.target.value)
        }))
        break;
      case Stats.intelligence:
        console.log("select intelligence")
        statListProps.setStatListData(currVal => ({
          ...currVal,
          intelligence: parseInt(event.target.value)
        }))
        break;
      case Stats.faith:
        console.log("select faith")
        statListProps.setStatListData(currVal => ({
          ...currVal,
          faith: parseInt(event.target.value)
        }))
        break;
      case Stats.arcane:
        console.log("select arcane")
        statListProps.setStatListData(currVal => ({
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
      defaultValue={target_stat} // TODO: needs to be generic
      onInput={(e: any) => {
          e.target.value = e.target.value.replace(/[^0-9]/g, "")
      }}
      inputProps={{ inputMode: 'numeric', pattern: '[0-9]*', style: { height: "17px", width: "46px" }}}
    />
  );
}

interface LevelIncrementerProps {
  target_stat: number;
  stat: string;
  min?: number;
  max?: number;
  statListProps: StatFullProps;
}

const LevelIncrementer: React.FC<LevelIncrementerProps> = ({ target_stat, stat, min, max, statListProps }) => {
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
            <Typography variant="body1">{stat}</Typography>
          </div>
        </Box>
        <IncrementerField
          stat={stat}
          target_stat={target_stat}
          key={target_stat} // TODO: needs to be generic
          statListProps={statListProps}
          min={min ? min : 0}
          max={max ? max : 99}
        />
      </Stack>
    </Box>
  );
}


// statlist component
// users should be able to change the props entered here. BUT they should not go beyond starter class
const StatList: React.FC<StatFullProps> = ({ statListData, setStatListData }) => {
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
          <LevelIncrementer 
            stat={Stats.level}
            target_stat={statListData.level}
            max={713}
            min={statListData.min_level}
            statListProps={{statListData, setStatListData}}
          />
          {/*has individual size components*/}
          <LevelIncrementer 
            stat={Stats.vigor}
            target_stat={statListData.vigor}
            min={statListData.min_vigor}
            statListProps={{statListData, setStatListData}}
          />
          <LevelIncrementer 
            stat={Stats.mind}
            target_stat={statListData.mind}
            min={statListData.min_mind}
            statListProps={{statListData, setStatListData}}
          />
          <LevelIncrementer 
            stat={Stats.endurance}
            target_stat={statListData.endurance}
            min={statListData.min_endurance}
            statListProps={{statListData, setStatListData}}
          />
          <LevelIncrementer 
            stat={Stats.strength}
            target_stat={statListData.strength}
            min={statListData.min_strength}
            statListProps={{statListData, setStatListData}}
          />
          <LevelIncrementer 
            stat={Stats.dexterity}
            target_stat={statListData.dexterity}
            min={statListData.min_dexterity}
            statListProps={{statListData, setStatListData}}
          />
          <LevelIncrementer 
            stat={Stats.intelligence}
            target_stat={statListData.intelligence}
            min={statListData.min_intelligence}
            statListProps={{statListData, setStatListData}}
          />
          <LevelIncrementer 
            stat={Stats.faith}
            target_stat={statListData.faith}
            min={statListData.min_faith}
            statListProps={{statListData, setStatListData}}
          />
          <LevelIncrementer 
            stat={Stats.arcane}
            target_stat={statListData.arcane}
            min={statListData.min_arcane}
            statListProps={{statListData, setStatListData}}
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
