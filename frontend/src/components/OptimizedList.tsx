import Box from "@mui/material/Box";
import Stack from "@mui/material/Stack";
import Typography from "@mui/material/Typography";
import React from 'react';
import { OptimizeListProps } from '../pages/Home'

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

interface LevelProps {
  stat_name: string;
  og_stat_val: number;
  stat_val: number;
}

const Level: React.FC<LevelProps> = ({ stat_name, og_stat_val, stat_val }) => {
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
            <Typography variant="body1">{stat_name}</Typography>
          </div>
        </Box>
        <Typography variant="body1">{og_stat_val} -- {stat_val} </Typography>
      </Stack>
    </Box>
  );
}

const OptimizedList: React.FC<OptimizeListProps> = ({ statlist, optimizelist }) => {
  return(
     <div>
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
          <Level stat_name={Stats.vigor} stat_val={optimizelist.vigor} og_stat_val={statlist.vigor} />
          <Level stat_name={Stats.mind} stat_val={optimizelist.mind} og_stat_val={statlist.mind} />
          <Level stat_name={Stats.endurance} stat_val={optimizelist.endurance} og_stat_val={statlist.endurance} />
          <Level stat_name={Stats.strength} stat_val={optimizelist.strength} og_stat_val={statlist.strength} />
          <Level stat_name={Stats.dexterity} stat_val={optimizelist.dexterity} og_stat_val={statlist.dexterity} />
          <Level stat_name={Stats.intelligence} stat_val={optimizelist.intelligence} og_stat_val={statlist.intelligence} />
          <Level stat_name={Stats.faith} stat_val={optimizelist.faith} og_stat_val={statlist.faith} />
          <Level stat_name={Stats.arcane} stat_val={optimizelist.arcane} og_stat_val={statlist.arcane} />
        </Stack>
      </Box>
    </div>
  );
}

export default OptimizedList;
