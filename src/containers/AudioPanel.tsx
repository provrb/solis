import { useState } from "react";
import CustomDropdown from "../components/Dropdown";
import { invoke } from "@tauri-apps/api/core";
import Header from "../components/Header";
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

function AudioSettingsPanel() {
  const [isRecording, setIsRecording] = useState(false);

  const handleRecordClick = () => {
    if (isRecording) {
      console.log("Stopped recording");
      invoke("stop_audio_recording");
    } else {
      console.log("Started recording");
      invoke("start_audio_recording");
    }

    setIsRecording(!isRecording);
  };

  const inputDeviceList = [{ value: "Default", label: "Default" }];
  const outputDeviceList = [{ value: "Default", label: "Default" }];
  const [inputDevice, setInputDevice] = useState("Default");
  const [outputDevice, setOutputDevice] = useState("Default");
  return (
    <div className="h-full bg-slate-50">
      <Header
        title="Audio Settings"
        subtitle="Configure your input and output devices"
      />
      <div className="px-6 space-y-4">
        {/* IO device selection dropdowns*/}
        <div className="flex space-x-3">
          {/* Inputt Devices */}
          <div className="flex-1">
            <label className="block text-sm font-medium text-slate-700 mb-2 font-montserrat">
              Input Device
            </label>
            <CustomDropdown
              value={inputDevice}
              onChange={setInputDevice}
              options={inputDeviceList}
              placeholder="Select input device"
            />
          </div>

          {/* Output Devices */}
          <div className="flex-1">
            <label className="block text-sm font-medium text-slate-700 mb-2 font-montserrat">
              Output Device
            </label>
            <CustomDropdown
              value={outputDevice}
              onChange={setOutputDevice}
              options={outputDeviceList}
              placeholder="Select output device"
            />
          </div>
        </div>

        {/*Input and output volume sliders*/}

        <div className="flex space-x-3">
          <div className="flex-1">
            <label className="block text-sm font-medium text-slate-700 mb-2 font-montserrat">
              Input Volume
            </label>
            <VolumeSlider
              defaultValue={90}
              aria-label="Volume"
              valueLabelDisplay="auto"
              valueLabelFormat={(value) => `${value}%`}
            />
          </div>
          <div className="flex-1">
            <label className="block text-sm font-medium text-slate-700 mb-2 font-montserrat">
              Output Volume
            </label>
            <VolumeSlider
              defaultValue={90}
              aria-label="Volume"
              valueLabelDisplay="auto"
              valueLabelFormat={(value) => `${value}%`}
            />
          </div>
        </div>
      </div>
      <button onClick={handleRecordClick}>
        {isRecording ? "Stop Recording" : "Start Recording"}
      </button>
    </div>
  );
}

export default AudioSettingsPanel;
