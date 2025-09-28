import Slider from "@mui/material/Slider";
import { styled } from "@mui/material/styles";

const VolumeSlider = styled(Slider)({
  color: "#303030",
  height: 6,
  padding: 0,
  "& .MuiSlider-track": {
    border: "none",
  },
  "& .MuiSlider-thumb": {
    height: 18,
    width: 18,
    backgroundColor: "#fff",
    border: "2px solid currentColor",
    "&:focus, &:hover, &.Mui-active, &.Mui-focusVisible": {
      boxShadow: "inherit",
    },
    "&::before": {
      display: "none",
    },
  },
  "& .MuiSlider-valueLabel": {
    fontSize: 13,
    backgroundColor: "#505050",
    borderRadius: 7,
    padding: "7px 9px 2px 9px",
    justifyContent: "center",
  },
});

export default VolumeSlider;
