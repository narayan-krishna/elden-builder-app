import Box from "@mui/material/Box";
import VirtualAutocomplete from './VirtualAutocomplete';
import Stack from "@mui/material/Stack";
import { LevelIncrementer } from "./Incrementer"
import { WeaponFullProps } from "../pages/Home"

const WeaponBox: React.FC<WeaponFullProps> = ({ weaponprops, setweaponprops }) => {

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
          <VirtualAutocomplete weaponData={weaponprops} setWeaponData={setweaponprops}/>
          <LevelIncrementer 
            stat={"Upgrade level"}
            target_stat={weaponprops.upgrade_lvl}
            max={weaponprops.max_upgrade_lvl}
            data={weaponprops}
            setData={setweaponprops}
          />
        </Stack>
      </Box>
    </div>
  );
}

export { WeaponBox as default }
