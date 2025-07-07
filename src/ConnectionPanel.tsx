import React, { useState, useRef, useEffect } from "react";
import { ChevronDown } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";

// Custom Dropdown Component
interface CustomDropdownProps {
  value: string;
  onChange: (value: string) => void;
  options: { value: string; label: string }[];
  placeholder?: string;
  className?: string;
}

interface ConnectionPanelProps {
  isConnected: boolean;
  setIsConnected: React.Dispatch<React.SetStateAction<boolean>>;
}

function CustomDropdown({
  value,
  onChange,
  options,
  placeholder,
  className = "",
}: CustomDropdownProps) {
  const [isOpen, setIsOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        dropdownRef.current &&
        !dropdownRef.current.contains(event.target as Node)
      ) {
        setIsOpen(false);
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const selectedOption = options.find((option) => option.value === value);

  return (
    <div className={`relative ${className}`} ref={dropdownRef}>
      {/* Dropdown Button */}
      <button
        type="button"
        onClick={() => setIsOpen(!isOpen)}
        className="w-full px-3 py-2 border border-slate-300 rounded-md focus:outline-none focus:ring-2 focus:ring-black focus:border-transparent font-montserrat bg-white flex items-center justify-between hover:border-slate-400 transition-colors duration-150"
      >
        <span className={selectedOption ? "text-slate-900" : "text-slate-500"}>
          {selectedOption ? selectedOption.label : placeholder}
        </span>
        <ChevronDown
          size={16}
          className={`text-slate-400 transition-transform duration-200 ${isOpen ? "rotate-180" : ""}`}
        />
      </button>

      {/* Dropdown Menu */}
      {isOpen && (
        <div className="absolute z-10 w-full mt-1 bg-white border border-slate-200 rounded-md shadow-lg max-h-60 overflow-auto">
          {options.map((option) => (
            <button
              key={option.value}
              type="button"
              onClick={() => {
                onChange(option.value);
                setIsOpen(false);
              }}
              className={`w-full px-3 py-2 text-left hover:bg-slate-50 transition-colors duration-150 font-montserrat ${
                option.value === value
                  ? "bg-blue-50 text-blue-600"
                  : "text-slate-700"
              }`}
            >
              {option.label}
            </button>
          ))}
        </div>
      )}
    </div>
  );
}

function ConnectionPanel({
  isConnected,
  setIsConnected,
}: ConnectionPanelProps) {
  const [udpServer, setUdpServer] = useState("127.0.0.1");
  const [udpPort, setUdpPort] = useState("20777");
  const [packetFormat, setPacketFormat] = useState("2022");

  // Packet format options
  const packetFormatOptions = [
    { value: "2022", label: "F1 2022" },
    { value: "2021", label: "F1 2021" },
    { value: "2020", label: "F1 2020" },
    { value: "2019", label: "F1 2019" },
  ];

  const handleConnect = () => {
    invoke("start_udp_listener", { address: udpServer, port: udpPort })
      .then((result) => {
        setIsConnected(Boolean(result));
        console.log("connected", isConnected);
      })
      .catch(() => {
        setIsConnected(false);
      });
  };

  const handleDisconnect = () => {
    invoke("stop_udp_listener");
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
            disabled={isConnected}
            className={`py-2 px-12 rounded-md transition-colors duration-200 font-medium font-montserrat ${
              isConnected
                ? "bg-gray-300 text-gray-500 cursor-not-allowed"
                : "bg-black text-white hover:bg-gray-800"
            }`}
          >
            Connect
          </button>
          <button
            onClick={handleDisconnect}
            disabled={!isConnected}
            className={`py-2 px-12 rounded-md transition-colors duration-200 font-medium font-montserrat ${
              !isConnected
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
