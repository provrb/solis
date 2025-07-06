import React, { useState } from "react";

function ConnectionPanel() {
  const [udpServer, setUdpServer] = useState("127.0.0.1");
  const [udpPort, setUdpPort] = useState("20777");
  const [packetFormat, setPacketFormat] = useState("2022");
  const [isConnected, setIsConnected] = useState(false);

  const handleConnect = () => {
    setIsConnected(true);
    // TODO: Implement actual connection logic
    console.log("Connecting to:", udpServer, udpPort, packetFormat);
  };

  const handleDisconnect = () => {
    setIsConnected(false);
    // TODO: Implement actual disconnection logic
    console.log("Disconnecting");
  };

  return (
    <div className="h-full bg-slate-50">
      {/* Header */}
      <div className="px-6 pt-6 pb-3">
        <h1 className="text-2xl font-bold text-black font-montserrat">Connection Settings</h1>
        <p className="text-md font-medium text-slate-700 font-montserrat mt-0">
          {isConnected ? 'Connected to UDP Server' : 'No connection established'}
        </p>
      </div>

      {/* Content */}
      <div className="px-6 space-y-3">
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
              className="w-full px-3 py-2 border border-slate-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent font-montserrat"
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
              className="w-full px-3 py-2 border border-slate-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent font-montserrat"
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
            <select
              value={packetFormat}
              onChange={(e) => setPacketFormat(e.target.value)}
              className="w-full px-2 pr-12 py-2 border border-slate-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent font-montserrat"
            >
              <option value="2022">2022</option>
            </select>
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
                ? 'bg-gray-300 text-gray-500 cursor-not-allowed' 
                : 'bg-black text-white hover:bg-gray-800'
            }`}
          >
            Connect
          </button>
          <button
            onClick={handleDisconnect}
            disabled={!isConnected}
            className={`py-2 px-12 rounded-md transition-colors duration-200 font-medium font-montserrat ${
              !isConnected 
                ? 'bg-gray-300 text-gray-500 cursor-not-allowed' 
                : 'bg-black text-white hover:bg-gray-800'
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