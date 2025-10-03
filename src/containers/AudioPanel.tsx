import { useState, useEffect } from "react";
import CustomDropdown from "../components/Dropdown";
import { invoke } from "@tauri-apps/api/core";
import Header from "../components/Header";
import VolumeSlider from "../components/VolumeSlider";
import "./App.css";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import type { Event as TauriEvent } from "@tauri-apps/api/event";

// events
type TranscribeEvent = {
  new_text: string;
};

function AudioSettingsPanel() {
  const [isRecording, setIsRecording] = useState(false);

  const handleRecordClick = () => {
    setIsRecording(!isRecording);
    if (isRecording) {
      console.log("Stopped recording");
      invoke("stop_audio_recording");
    } else {
      console.log("Started recording");
      invoke("start_audio_recording");
    }
  };

  const [inputDeviceList, setInputDeviceList] = useState<
    { value: string; label: string }[]
  >(() => {
    const stored = localStorage.getItem("inputDeviceList");
    return stored
      ? JSON.parse(stored)
      : [{ value: "Default", label: "Default" }];
  });

  const [outputDeviceList, setOutputDeviceList] = useState<
    { value: string; label: string }[]
  >(() => {
    const stored = localStorage.getItem("outputDeviceList");
    return stored
      ? JSON.parse(stored)
      : [{ value: "Default", label: "Default" }];
  });

  const [inputDevice, _setInputDevice] = useState(
    () => localStorage.getItem("inputDevice") || "Default",
  );

  const [outputDevice, _setOutputDevice] = useState(
    () => localStorage.getItem("outputDevice") || "Default",
  );

  const setInputDevice = (deviceName: string) => {
    setIsRecording(false);
    invoke("set_input_device", { deviceName });
    _setInputDevice(deviceName);
    localStorage.setItem("inputDevice", deviceName);
  };

  const setOutputDevice = (deviceName: string) => {
    _setOutputDevice(deviceName);
    localStorage.setItem("outputDevice", deviceName);
  };

  useEffect(() => {
    const fetchDevices = async () => {
      const inputs = await invoke<string[]>("get_input_devices");
      const outputs = await invoke<string[]>("get_output_devices");

      setInputDeviceList((prev) => {
        const merged = [
          ...prev,
          ...inputs.map((d) => ({ value: d, label: d })),
        ].filter(
          (item, index, arr) =>
            arr.findIndex((x) => x.value === item.value) === index,
        );

        localStorage.setItem("inputDeviceList", JSON.stringify(merged));

        if (!merged.find((d) => d.value === inputDevice) && merged.length > 0) {
          setInputDevice(merged[0].value);
        }

        return merged;
      });

      setOutputDeviceList((prev) => {
        const merged = [
          ...prev,
          ...outputs.map((d) => ({ value: d, label: d })),
        ].filter(
          (item, index, arr) =>
            arr.findIndex((x) => x.value === item.value) === index,
        );

        localStorage.setItem("outputDeviceList", JSON.stringify(merged));

        if (
          !merged.find((d) => d.value === outputDevice) &&
          merged.length > 0
        ) {
          setOutputDevice(merged[0].value);
        }

        return merged;
      });
    };

    fetchDevices();
  }, []);
  const setInputVolume = (
    _event: Event,
    newValue: number | number[],
    _activeThumb: number,
  ) => {
    console.log("setting input volume:", newValue);
    invoke("set_input_volume", { newVolume: newValue });
  };
  const setOutputVolume = (
    _event: Event,
    newValue: number | number[],
    _activeThumb: number,
  ) => {
    invoke("set_output_volume", { newVolume: newValue });
  };

  const webviewWindow = getCurrentWebviewWindow();
  useEffect(() => {
    const transcribeBox = document.getElementById("transcribe-box");
    const unlisten = webviewWindow.listen(
      "append-transcribed-text",
      (event: TauriEvent<TranscribeEvent>) => {
        console.log("received event", event);
        if (transcribeBox) {
          transcribeBox.textContent += event.payload.new_text;
        }
      },
    );

    return () => {
      unlisten.then((f) => f());
    };
  }, [webviewWindow]);

  return (
    <div className="h-full bg-slate-50 overflow-y-auto">
      <Header
        title="Audio Settings"
        subtitle="Configure your input and output devices"
      />
      <div className="px-6 space-y-4">
        {/* IO device selection dropdowns*/}
        <div className="flex space-x-3">
          {/* Input Devices */}
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
              defaultValue={100}
              aria-label="Volume"
              valueLabelDisplay="auto"
              onChange={setInputVolume}
              valueLabelFormat={(value) => `${value}%`}
            />{" "}
          </div>
          <div className="flex-1">
            <label className="block text-sm font-medium text-slate-700 mb-2 font-montserrat">
              Output Volume
            </label>
            <VolumeSlider
              defaultValue={100}
              onChange={setOutputVolume}
              aria-label="Volume"
              valueLabelDisplay="auto"
              valueLabelFormat={(value) => `${value}%`}
              max={150}
            />
          </div>
        </div>
        <div className="flex-1 h-full">
          <label className="block text-sm font-medium text-slate-700 mb-2 font-montserrat">
            Test Text-To-Speech
          </label>
          <textarea
            id="transcribe-box"
            placeholder="Transcribed text will appear here."
            className="w-full px-3 py-2 font-['Fira_Code'] border border-slate-200 rounded-md text-xs h-48"
            readOnly={true}
          ></textarea>
          <div className="flex space-x-3 pt-3">
            <button
              className={`py-2 px-12 rounded-md transition-colors duration-200 font-medium font-montserrat ${
                isRecording
                  ? "bg-gray-300 text-gray-500 cursor-not-allowed"
                  : "bg-black text-white hover:bg-gray-800"
              }`}
              onClick={handleRecordClick}
              disabled={isRecording}
            >
              Start Recording
            </button>

            <button
              className={`py-2 px-12 rounded-md transition-colors duration-200 font-medium font-montserrat ${
                !isRecording
                  ? "bg-gray-300 text-gray-500 cursor-not-allowed"
                  : "bg-black text-white hover:bg-gray-800"
              }`}
              onClick={handleRecordClick}
            >
              Stop Recording
            </button>
          </div>
        </div>
      </div>
      {/* <button onClick={handleRecordClick}> */}
      {/*   {isRecording ? "Stop Recording" : "Start Recording"} */}
      {/* </button> */}
    </div>
  );
}

export default AudioSettingsPanel;
