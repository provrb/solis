import React, { useState, useRef, useEffect } from "react";
import CustomDropdown from "../components/Dropdown";
import { invoke } from "@tauri-apps/api/core";
import { emit } from "@tauri-apps/api/event";

interface ConnectionPanelProps {
  isConnected: boolean;
  setIsConnected: React.Dispatch<React.SetStateAction<boolean>>;
}

function ConnectionPanel({
  isConnected,
  setIsConnected,
}: ConnectionPanelProps) {
  const [udpServer, setUdpServer] = useState("127.0.0.1");
  const [udpPort, setUdpPort] = useState("20777");
  const [packetFormat, setPacketFormat] = useState("2022");
  const [connecting, setConnecting] = useState(false);

  // Packet format options
  const packetFormatOptions = [
    { value: "2022", label: "F1 2022" },
    { value: "2021", label: "F1 2021" },
    { value: "2020", label: "F1 2020" },
    { value: "2019", label: "F1 2019" },
  ];

  const handleConnect = () => {
    setConnecting(true);

    invoke("start_udp_listener", { address: udpServer, port: udpPort })
      .then((result) => {
        setIsConnected(Boolean(result));
        console.log("connected", Boolean(result));
      })
      .catch(() => {
        setIsConnected(false);
      })
      .finally(() => {
        setConnecting(false);
      });
  };

  const handleDisconnect = () => {
    emit("stop_udp_listener");
    setIsConnected(false);
  };

  return (
    <div className="h-full bg-slate-50">
      {/* Header */}
      <div className="px-6 pt-6 pb-3">
        <h1 className="text-2xl font-bold text-black font-montserrat">
          Connection Settings
        </h1>
        <p className="text-md font-medium text-slate-700 font-montserrat mt-0">
          {isConnected
            ? "Connected to UDP Server"
            : "No connection established"}
        </p>
      </div>

      {/* Content */}
      <div className="px-6 space-y-4">
        {/* UDP Server and Port Row */}
        <div className="flex space-x-3">
          {/* UDP Server */}
          <div className="flex-1">
            <label className="block text-sm font-medium text-slate-700 mb-2 font-montserrat">
              UDP Server
            </label>
            <input
              type="text"
              value={udpServer}
              onChange={(e) => setUdpServer(e.target.value)}
              className="w-full px-3 py-2 border border-slate-300 rounded-md focus:outline-none focus:ring-2 focus:ring-black focus:border-transparent font-montserrat"
              placeholder="127.0.0.1"
            />
          </div>

          {/* UDP Port */}
          <div className="flex-1">
            <label className="block text-sm font-medium text-slate-700 mb-2 font-montserrat">
              UDP Port
            </label>
            <input
              type="number"
              value={udpPort}
              onChange={(e) => setUdpPort(e.target.value)}
              className="w-full px-3 py-2 border border-slate-300 rounded-md focus:outline-none focus:ring-2 focus:ring-black focus:border-transparent font-montserrat"
              placeholder="20777"
            />
          </div>
        </div>

        {/* Packet Format */}
        <div className="flex space-x-3">
          <div className="flex-1">
            <label className="block text-sm font-medium text-slate-700 mb-2 font-montserrat">
              Packet Format
            </label>
            <CustomDropdown
              value={packetFormat}
              onChange={setPacketFormat}
              options={packetFormatOptions}
              placeholder="Select packet format"
            />
          </div>
          <div className="flex-1"></div>
        </div>

        {/* Action Buttons */}
        <div className="flex space-x-3 pt-2">
          <button
            onClick={handleConnect}
            disabled={isConnected || connecting}
            className={`py-2 px-12 rounded-md transition-colors duration-200 font-medium font-montserrat ${isConnected || connecting
                ? "bg-gray-300 text-gray-500 cursor-not-allowed"
                : "bg-black text-white hover:bg-gray-800"
              }`}
          >
            Connect
          </button>

          <button
            onClick={handleDisconnect}
            disabled={!isConnected || connecting}
            className={`py-2 px-12 rounded-md transition-colors duration-200 font-medium font-montserrat ${!isConnected || connecting
                ? "bg-gray-300 text-gray-500 cursor-not-allowed"
                : "bg-black text-white hover:bg-gray-800"
              }`}
          >
            Disconnect
          </button>
        </div>
      </div>
    </div>
  );
}

export default ConnectionPanel;
