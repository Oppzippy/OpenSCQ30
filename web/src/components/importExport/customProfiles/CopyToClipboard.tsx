import { Typography } from "@mui/material";
import { CopyToClipboardState } from "./ExportCustomProfilesState";

export function CopyToClipboard({ state }: { state: CopyToClipboardState }) {
  return <Typography>{state.profileString}</Typography>;
}
