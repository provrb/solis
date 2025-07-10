import React, { useState } from "react";
import CustomDropdown from "../components/Dropdown";
import { invoke } from "@tauri-apps/api/core";
import { emit } from "@tauri-apps/api/event";

function AudioSettingsPanel() {
  const inputDevices = invoke("get_audio_inp_devices");
  const outputDevices = invoke("get_audio_out_devices");
}

export default AudioSettingsPanel;