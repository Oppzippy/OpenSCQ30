import {
  Checkbox,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
} from "@mui/material";
import {
  ExportCustomProfilesState,
  ProfileSelectionState,
} from "./ExportCustomProfilesState";

export function CustomProfileSelection({
  state,
  onStateChange,
}: {
  state: ProfileSelectionState;
  onStateChange: (state: ExportCustomProfilesState) => void;
}) {
  function toggle(index: number) {
    const newSelection = [...state.selection];
    newSelection[index] = !newSelection[index];
    onStateChange({
      ...state,
      selection: newSelection,
    });
  }

  return (
    <List>
      {state.profiles.map((profile, i) => {
        const labelId = `export-custom-profile-selection-${i}`;
        return (
          <ListItem key={i} disablePadding>
            <ListItemButton onClick={() => toggle(i)} dense>
              <ListItemIcon>
                <Checkbox
                  edge="start"
                  checked={state.selection[i]}
                  disableRipple
                  aria-labelledby={labelId}
                />
              </ListItemIcon>
              <ListItemText id={labelId} primary={profile.name} />
            </ListItemButton>
          </ListItem>
        );
      })}
    </List>
  );
}
