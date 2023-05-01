import Box from "@mui/material/Box";
import Stack from "@mui/material/Stack";
import Typography from "@mui/material/Typography";
import ArrowRightAltIcon from '@mui/icons-material/ArrowRightAlt';

const Stats = {
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

function Level(props) {
  // TODO: this should be a combination of a text field and a selector
  return (
    <Box
      sx={{
        width: 350,
        height: 25,
        p: .5,
      }}
      justifyContent="center"
      alignItems="center"
    >
      <Stack direction="row" spacing={2}>
        <Box display="flex" justifyContent="left" alignItems="flex-end" sx={{flexGrow: 1, p: 0, }}>
          <div>
            {console.log(props.stat)}
            <Typography variant="body1">{props.stat_name}</Typography>
          </div>
        </Box>
        <Typography variant="body1">{props.og_stat_val} -- {props.stat_val} </Typography>
      </Stack>
    </Box>
  );
}

function GreedyRender(props) {
  return (
    <div>
      <div>{props.optimizedData.level}</div>
      <div>{props.optimizedData.vigor}</div>
      <div>{props.optimizedData.mind}</div>
      <div>{props.optimizedData.endurance}</div>
      <div>{props.optimizedData.strength}</div>
      <div>{props.optimizedData.dexterity}</div>
      <div>{props.optimizedData.intelligence}</div>
      <div>{props.optimizedData.faith}</div>
      <div>{props.optimizedData.arcane}</div>
    </div>
  );
}

function OptimizedList(props) {
  return(
    props.optimizedData && <div>
      <Box
        sx={{
          display: 'flex',
          alignItems: 'center',
          m: 0,
          p: 2,
          bgcolor: 'background.paper',
          borderColor: 'grey.500',
          border: 1,
          borderRadius: 1,
        }}
        border={0}
        borderLeft={1}
        borderRight={1}
      >
        <Stack spacing={{ xs: .1, sm: .4 }} direction="column" useFlexGap flexWrap="wrap" alignItems="center" justifyContent="center">
          <Level stat_name={Stats.vigor} stat_val={props.optimizedData.vigor} og_stat_val={props.statListData.vigor} />
          <Level stat_name={Stats.mind} stat_val={props.optimizedData.mind} og_stat_val={props.statListData.mind} />
          <Level stat_name={Stats.endurance} stat_val={props.optimizedData.endurance} og_stat_val={props.statListData.endurance} />
          <Level stat_name={Stats.strength} stat_val={props.optimizedData.strength} og_stat_val={props.statListData.strength} />
          <Level stat_name={Stats.dexterity} stat_val={props.optimizedData.dexterity} og_stat_val={props.statListData.dexterity} />
          <Level stat_name={Stats.intelligence} stat_val={props.optimizedData.intelligence} og_stat_val={props.statListData.intelligence} />
          <Level stat_name={Stats.faith} stat_val={props.optimizedData.faith} og_stat_val={props.statListData.faith} />
          <Level stat_name={Stats.arcane} stat_val={props.optimizedData.arcane} og_stat_val={props.statListData.arcane} />
        </Stack>
      </Box>
    </div>
  );
}

export default OptimizedList;
