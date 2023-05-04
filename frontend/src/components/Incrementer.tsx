import Stack from "@mui/material/Stack";
import Box from "@mui/material/Box";
import TextField from "@mui/material/TextField";
import Typography from '@mui/material/Typography';
import { WeaponProps } from '../pages/Home'

interface IncFieldProps {
  min: number;
  max: number;
  target_stat: number;
  setData: React.Dispatch<React.SetStateAction<WeaponProps>>;
};

interface LevelIncProps {
  stat: string;
  target_stat: number;
  min?: number;
  max?: number;
  data: WeaponProps;
  setData: React.Dispatch<React.SetStateAction<WeaponProps>>
}

// this should be more general purpose
export const IncrementerField: React.FC<IncFieldProps> = ({ target_stat, min, max, setData }) => {
  const handleLeave = (event: any) => {
    // if the field is left blank, or is below the minimum or above max
    if (event.target.value === "" || parseInt(event.target.value) < min || parseInt(event.target.value) > max) {
      console.log("invalid stat, reverted to:", target_stat)
      // TODO: spawn an error here for the user
      event.target.value = target_stat
    }
    
    setData(currVal => ({
      ...currVal,
      upgrade_lvl: parseInt(event.target.value)
    }))
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

export const LevelIncrementer: React.FC<LevelIncProps> = ({ stat, target_stat, min, max, setData }) => {
  // TODO: this should be a combination of a text field and a selector

  return (
    <Box
      sx={{
        width: 350,
        height: 50,
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
          target_stat={target_stat}
          key={target_stat} // TODO: needs to be generic
          setData={setData}
          min={min ? min : 0}
          max={max ? max : 99}
        />
      </Stack>
    </Box>
  );
}
