import { useState } from 'react';
import './App.css';
import StatList from './components/StatList'
import { get_info } from './services/profile'
import Box from "@mui/material/Box";
import Stack from "@mui/material/Stack";
import WeaponBox from './components/Weapon'
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button'

function App() {

  // TODO: make this its own page (setup/configuration) and make weapon and stat state global
  return (
    <div className="App">
      <Box
        minHeight='100vh'
        display='flex'
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
            <StatList />
          </div>
          <div style={{display: 'flex', justifyContent:'center'}}>
            <WeaponBox />
          </div>
          <Box
            display='flex'
            sx={{
              p: 1,
            }}
            justifyContent="center"
            alignItems="center"
          >
            <Button onClick={() => {}} variant="contained">Optimize</Button>
          </Box>
        </Stack>
      </Box>
    </div>
  );
}

export default App;
