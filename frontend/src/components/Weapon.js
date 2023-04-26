import Box from "@mui/material/Box";
import commonStyles from './common'
import { useState } from 'react';
import VirtualAutocomplete from './VirtualAutocomplete';
import Stack from "@mui/material/Stack";
import LevelIncrementer from "./Incrementer"

function WeaponBox() {
  const [weaponData, setWeaponData] = useState({
    name: "Ruins Greatsword",
    upgrade_lvl: 5,
    max_upgrade_lvl: 10,
  })

  return (
    <div>
      <Box
        sx={{
          display: 'flex',
          justifyContent: 'center',
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
        <Stack spacing={{ xs: .1, sm: 1.65 }} direction="column" useFlexGap flexWrap="wrap" alignItems="center" justifyContent="center">
          <VirtualAutocomplete weaponData={weaponData} setWeaponData={setWeaponData}/>
          <LevelIncrementer 
            stat={"Upgrade level"}
            target_stat={weaponData.upgrade_lvl}
            max={weaponData.max_upgrade_lvl}
            data={weaponData}
            setData={setWeaponData}
          />
        </Stack>
      </Box>
    </div>
  );
}

export { WeaponBox as default }
