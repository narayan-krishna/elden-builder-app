import Stack from "@mui/material/Stack";
import Box from "@mui/material/Box";
import TextField from "@mui/material/TextField";
import Typography from '@mui/material/Typography';

// this should be more general purpose
function IncrementerField(props) {
  const handleLeave = (event) => {
    // if the field is left blank, or is below the minimum or above max
    if (event.target.value === "" || parseInt(event.target.value) < props.min || parseInt(event.target.value) > props.max) {
      console.log("invalid stat, reverted to:", props.target_stat)
      // TODO: spawn an error here for the user
      event.target.value = props.target_stat
    }
    
    props.setData(currVal => ({
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
      defaultValue={props.target_stat} // TODO: needs to be generic
      onInput={(e) => {
          e.target.value = e.target.value.replace(/[^0-9]/g, "")
      }}
      inputProps={{ inputMode: 'numeric', pattern: '[0-9]*', style: { height: "17px", width: "46px" }}}
    />
  );
}

export default function LevelIncrementer(props) {
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
            <Typography variant="body1">{props.stat}</Typography>
          </div>
        </Box>
        <IncrementerField
          target_stat={props.target_stat}
          key={props.target_stat} // TODO: needs to be generic
          setData={props.setData}
          data={props.data}
          min={props.min}
          max={props.max}
        />
      </Stack>
    </Box>
  );
}

LevelIncrementer.defaultProps = {
  min: 1,
  max: 99,
}
