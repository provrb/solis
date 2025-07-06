import React, { useState, useEffect } from "react";
import { ChevronDown, ChevronRight, MoreVertical, ArrowUp, ArrowDown, Trash2 } from "lucide-react";

interface DataRow {
  id: string;
  timestamp: string;
  packetId: string;
  expanded: boolean;
  showActions: boolean;
  rawData: string;
}

interface DataPanelProps {
  title: string;
  data?: DataRow[];
}

// Function to get subtitle based on panel type
const getSubtitle = (title: string): string => {
  switch (title) {
    case "Motion Data":
      return "Real-time physics data for all vehicles in the session";
    case "Session Data":
      return "Weather conditions, track information, and session details";
    case "Lap Data":
      return "Detailed lap times, sector data, and driver status";
    case "Events":
      return "Session events, incidents, and race flags";
    case "Car Setups":
      return "Aerodynamic, suspension, and brake configurations";
    case "Car Telemetry":
      return "Live vehicle data including speed, RPM, and temperatures";
    case "Car Status":
      return "Fuel levels, DRS status, and traction control settings";
    case "Car Damage":
      return "Component wear, damage levels, and fault indicators";
    case "Participants":
      return "Driver information and participant details";
    case "Lobby Info":
      return "Multiplayer lobby settings and player status";
    case "Final Classification":
      return "Race results, standings, and final lap times";
    case "Session History":
      return "Historical lap data and tire usage information";
    default:
      return "View and manage telemetry data";
  }
};

// Function to get sample data based on panel type
const getSampleData = (title: string): DataRow[] => {
  switch (title) {
    case "Motion Data":
      return [
        {
          id: "1",
          timestamp: "2024-01-15 14:30:25.123",
          packetId: "0x01",
          expanded: false,
          showActions: false,
          rawData: `Motion Data Packet:
  m_worldPositionX: 123.456
  m_worldPositionY: 789.012
  m_worldPositionZ: 345.678
  m_worldVelocityX: 45.67
  m_worldVelocityY: 89.01
  m_worldVelocityZ: 23.45
  m_worldForwardDirX: 0.707
  m_worldForwardDirY: 0.707
  m_worldForwardDirZ: 0.0
  m_worldRightDirX: -0.707
  m_worldRightDirY: 0.707
  m_worldRightDirZ: 0.0`
        },
        {
          id: "2",
          timestamp: "2024-01-15 14:30:25.456",
          packetId: "0x01",
          expanded: false,
          showActions: false,
          rawData: `Motion Data Packet:
  m_worldPositionX: 124.567
  m_worldPositionY: 790.123
  m_worldPositionZ: 346.789
  m_worldVelocityX: 46.78
  m_worldVelocityY: 90.12
  m_worldVelocityZ: 24.56
  m_worldForwardDirX: 0.708
  m_worldForwardDirY: 0.706
  m_worldForwardDirZ: 0.0
  m_worldRightDirX: -0.706
  m_worldRightDirY: 0.708
  m_worldRightDirZ: 0.0`
        }
      ];

    case "Session Data":
      return [
        {
          id: "1",
          timestamp: "2024-01-15 14:30:25.123",
          packetId: "0x02",
          expanded: false,
          showActions: false,
          rawData: `Session Data Packet:
  m_weather: 0
  m_trackTemperature: 25.5
  m_airTemperature: 22.3
  m_totalLaps: 20
  m_trackLength: 5000
  m_sessionType: 1
  m_trackId: 7
  m_formula: 0
  m_sessionTimeLeft: 1800
  m_sessionDuration: 3600
  m_pitSpeedLimit: 80
  m_gamePaused: 0
  m_isSpectating: 0
  m_spectatorCarIndex: 0`
        }
      ];

    case "Lap Data":
      return [
        {
          id: "1",
          timestamp: "2024-01-15 14:30:25.123",
          packetId: "0x03",
          expanded: false,
          showActions: false,
          rawData: `Lap Data Packet:
  m_lastLapTime: 89.123
  m_currentLapTime: 45.678
  m_sector1Time: 29.456
  m_sector2Time: 30.234
  m_lapDistance: 2500.5
  m_totalDistance: 12500.75
  m_safetyCarDelta: 0.0
  m_carPosition: 1
  m_currentLapNum: 5
  m_pitStatus: 0
  m_sector: 1
  m_currentLapInvalid: 0
  m_penalties: 0
  m_warnings: 0`
        }
      ];

    case "Events":
      return [
        {
          id: "1",
          timestamp: "2024-01-15 14:30:25.123",
          packetId: "0x04",
          expanded: false,
          showActions: false,
          rawData: `Event Packet:
  m_eventStringCode: "SSTA"
  m_eventDetails: "Session Started"
  m_eventType: 0
  m_eventTime: 123.456`
        },
        {
          id: "2",
          timestamp: "2024-01-15 14:30:30.456",
          packetId: "0x04",
          expanded: false,
          showActions: false,
          rawData: `Event Packet:
  m_eventStringCode: "CHQF"
  m_eventDetails: "Chequered Flag"
  m_eventType: 1
  m_eventTime: 128.789`
        }
      ];

    case "Car Setups":
      return [
        {
          id: "1",
          timestamp: "2024-01-15 14:30:25.123",
          packetId: "0x05",
          expanded: false,
          showActions: false,
          rawData: `Car Setup Packet:
  m_frontWing: 3
  m_rearWing: 2
  m_onThrottle: 85
  m_offThrottle: 65
  m_frontCamber: -2.5
  m_rearCamber: -1.0
  m_frontToe: 0.1
  m_rearToe: 0.3
  m_frontSuspension: 2
  m_rearSuspension: 1
  m_frontAntiRollBar: 3
  m_rearAntiRollBar: 2
  m_frontSuspensionHeight: 0
  m_rearSuspensionHeight: 0
  m_brakePressure: 85
  m_brakeBias: 58`
        }
      ];

    case "Car Telemetry":
      return [
        {
          id: "1",
          timestamp: "2024-01-15 14:30:25.123",
          packetId: "0x06",
          expanded: false,
          showActions: false,
          rawData: `Car Telemetry Packet:
  m_speed: 245.6
  m_throttle: 0.85
  m_steer: 0.12
  m_brake: 0.0
  m_clutch: 0.0
  m_gear: 6
  m_engineRPM: 12500
  m_drs: 0
  m_revLightsPercent: 85
  m_brakesTemperature: [120, 125, 118, 122]
  m_tyresSurfaceTemperature: [95, 98, 92, 96]
  m_tyresInnerTemperature: [88, 91, 85, 89]
  m_engineTemperature: 105
  m_tyresPressure: [23.5, 23.8, 23.2, 23.6]`
        }
      ];

    case "Car Status":
      return [
        {
          id: "1",
          timestamp: "2024-01-15 14:30:25.123",
          packetId: "0x07",
          expanded: false,
          showActions: false,
          rawData: `Car Status Packet:
  m_tractionControl: 1
  m_antiLockBrakes: 1
  m_fuelMix: 2
  m_frontBrakeBias: 58
  m_pitLimiterStatus: 0
  m_fuelInTank: 95.5
  m_fuelCapacity: 110.0
  m_fuelRemainingLaps: 15.2
  m_maxRPM: 12500
  m_idleRPM: 1000
  m_maxGears: 8
  m_drsAllowed: 1
  m_drsActivationDistance: 1000
  m_tyresWear: [15, 18, 12, 16]
  m_actualTyreCompound: 2
  m_visualTyreCompound: 2`
        }
      ];

    case "Car Damage":
      return [
        {
          id: "1",
          timestamp: "2024-01-15 14:30:25.123",
          packetId: "0x08",
          expanded: false,
          showActions: false,
          rawData: `Car Damage Packet:
  m_tyresWear: [15, 18, 12, 16]
  m_tyresDamage: [0, 0, 0, 0]
  m_brakesDamage: [0, 0, 0, 0]
  m_frontLeftWingDamage: 0
  m_frontRightWingDamage: 0
  m_rearWingDamage: 0
  m_floorDamage: 0
  m_diffuserDamage: 0
  m_sidepodDamage: 0
  m_drsFault: 0
  m_gearBoxDamage: 0
  m_engineDamage: 0
  m_engineMGUHWear: 0
  m_engineESWear: 0
  m_engineCEWear: 0
  m_engineICWear: 0`
        }
      ];

    case "Participants":
      return [
        {
          id: "1",
          timestamp: "2024-01-15 14:30:25.123",
          packetId: "0x09",
          expanded: false,
          showActions: false,
          rawData: `Participants Packet:
  m_numActiveCars: 20
  m_participants: [
    { m_aiControlled: 0, m_driverId: 44, m_teamId: 0, m_raceNumber: 44, m_nationality: 1, m_name: "HAM" },
    { m_aiControlled: 0, m_driverId: 77, m_teamId: 0, m_raceNumber: 77, m_nationality: 1, m_name: "BOT" },
    { m_aiControlled: 0, m_driverId: 33, m_teamId: 1, m_raceNumber: 33, m_nationality: 1, m_name: "VER" }
  ]`
        }
      ];

    case "Lobby Info":
      return [
        {
          id: "1",
          timestamp: "2024-01-15 14:30:25.123",
          packetId: "0x0A",
          expanded: false,
          showActions: false,
          rawData: `Lobby Info Packet:
  m_aiControlled: 0
  m_teamId: 0
  m_nationality: 1
  m_platform: 1
  m_customCarName: "HAM44"
  m_carSetup: 0
  m_carSetupName: "Default"
  m_carSetupWings: 0
  m_carSetupOnThrottle: 0
  m_carSetupOffThrottle: 0
  m_frontWing: 3
  m_rearWing: 2
  m_onThrottle: 85
  m_offThrottle: 65
  m_frontCamber: -2.5
  m_rearCamber: -1.0`
        }
      ];

    case "Final Classification":
      return [
        {
          id: "1",
          timestamp: "2024-01-15 14:30:25.123",
          packetId: "0x0B",
          expanded: false,
          showActions: false,
          rawData: `Final Classification Packet:
  m_numCars: 20
  m_classificationData: [
    { m_position: 1, m_numLaps: 20, m_gridPosition: 1, m_points: 25, m_numPitStops: 1, m_resultStatus: 0, m_bestLapTime: 89.123, m_totalRaceTime: 1800.456 },
    { m_position: 2, m_numLaps: 20, m_gridPosition: 2, m_points: 18, m_numPitStops: 1, m_resultStatus: 0, m_bestLapTime: 89.456, m_totalRaceTime: 1801.789 },
    { m_position: 3, m_numLaps: 20, m_gridPosition: 3, m_points: 15, m_numPitStops: 1, m_resultStatus: 0, m_bestLapTime: 89.789, m_totalRaceTime: 1802.123 }
  ]`
        }
      ];

    case "Session History":
      return [
        {
          id: "1",
          timestamp: "2024-01-15 14:30:25.123",
          packetId: "0x0C",
          expanded: false,
          showActions: false,
          rawData: `Session History Packet:
  m_carIdx: 0
  m_numLaps: 20
  m_numTyreStints: 2
  m_bestLapTimeLapNum: 5
  m_bestSector1LapNum: 3
  m_bestSector2LapNum: 7
  m_bestSector3LapNum: 12
  m_lapHistoryData: [
    { m_lapTime: 89.123, m_sector1Time: 29.456, m_sector2Time: 30.234, m_sector3Time: 29.433, m_lapValidBitFlags: 1 },
    { m_lapTime: 88.987, m_sector1Time: 29.123, m_sector2Time: 30.001, m_sector3Time: 29.863, m_lapValidBitFlags: 1 }
  ]
  m_tyreStintsHistoryData: [
    { m_endLap: 10, m_tyreActualCompound: 2, m_tyreVisualCompound: 2 },
    { m_endLap: 20, m_tyreActualCompound: 2, m_tyreVisualCompound: 2 }
  ]`
        }
      ];

    default:
      return [
        {
          id: "1",
          timestamp: "2024-01-15 14:30:25.123",
          packetId: "0x00",
          expanded: false,
          showActions: false,
          rawData: `Default Packet:
  m_packetFormat: 2022
  m_gameMajorVersion: 1
  m_gameMinorVersion: 23
  m_packetVersion: 1
  m_packetId: 0
  m_sessionUID: 12345678901234567890
  m_sessionTime: 123.456
  m_frameIdentifier: 12345
  m_playerCarIndex: 0
  m_secondaryPlayerCarIndex: 255`
        }
      ];
  }
};

function DataPanel({ title, data }: DataPanelProps) {
  // Use provided data or fallback to sample data
  const [dataRows, setDataRows] = useState<DataRow[]>(data || getSampleData(title));

  // Update data when title changes (if no custom data provided)
  useEffect(() => {
    if (!data) {
      setDataRows(getSampleData(title));
    }
  }, [title, data]);

  const toggleExpanded = (id: string) => {
    setDataRows(rows =>
      rows.map(row =>
        row.id === id ? { ...row, expanded: !row.expanded } : row
      )
    );
  };

  const toggleActions = (id: string) => {
    setDataRows(rows =>
      rows.map(row =>
        row.id === id ? { ...row, showActions: !row.showActions } : row
      )
    );
  };

  const deleteRow = (id: string) => {
    setDataRows(prevRows => prevRows.filter(row => row.id !== id));
    // Close actions menu after deletion
    setDataRows(prevRows => 
      prevRows.map(row => ({ ...row, showActions: false }))
    );
  };

  const moveRow = (id: string, direction: 'up' | 'down') => {
    setDataRows(prevRows => {
      const currentIndex = prevRows.findIndex(row => row.id === id);
      if (currentIndex === -1) return prevRows;

      const newRows = [...prevRows];
      
      if (direction === 'up' && currentIndex > 0) {
        // Move up: swap with previous row
        [newRows[currentIndex], newRows[currentIndex - 1]] = [newRows[currentIndex - 1], newRows[currentIndex]];
      } else if (direction === 'down' && currentIndex < newRows.length - 1) {
        // Move down: swap with next row
        [newRows[currentIndex], newRows[currentIndex + 1]] = [newRows[currentIndex + 1], newRows[currentIndex]];
      }
      
      // Close actions menu after moving
      return newRows.map(row => ({ ...row, showActions: false }));
    });
  };

  return (
    <div className="h-full bg-slate-50">
      {/* Header */}
      <div className="px-6 pt-6 pb-3">
        <h1 className="text-2xl font-bold text-black font-montserrat">{title}</h1>
        <p className="text-slate-600 font-montserrat mt-1">{getSubtitle(title)}</p>
      </div>

      {/* Data List */}
      <div className="px-6 flex-1 overflow-hidden">
        <div className="bg-white rounded-lg border border-slate-200 overflow-hidden">
          <div className="overflow-y-auto max-h-[calc(100vh-200px)]">
            {dataRows.map((row) => (
              <div key={row.id} className="border-b border-slate-100 last:border-b-0">
                {/* Main Row */}
                <div className="flex items-center px-4 py-3 hover:bg-slate-50 transition-colors duration-150">
                  {/* Dropdown Arrow */}
                  <button
                    onClick={() => toggleExpanded(row.id)}
                    className="mr-3 text-slate-400 hover:text-slate-600 transition-colors duration-150"
                  >
                    {row.expanded ? <ChevronDown size={16} /> : <ChevronRight size={16} />}
                  </button>

                  {/* Timestamp */}
                  <div className="text-sm text-slate-600 font-montserrat mr-4">
                    {row.timestamp}
                  </div>

                  {/* Packet ID */}
                  <div className="text-sm text-slate-500 font-montserrat mr-4">
                    {row.packetId}
                  </div>

                  {/* Spacer */}
                  <div className="flex-1"></div>

                  {/* Actions */}
                  <div className="flex items-center space-x-2">
                    {row.showActions && (
                      <>
                        <button
                          onClick={() => moveRow(row.id, 'up')}
                          className="text-slate-400 hover:text-slate-600 transition-colors duration-150 p-1"
                        >
                          <ArrowUp size={16} />
                        </button>
                        <button
                          onClick={() => moveRow(row.id, 'down')}
                          className="text-slate-400 hover:text-slate-600 transition-colors duration-150 p-1"
                        >
                          <ArrowDown size={16} />
                        </button>
                        <button
                          onClick={() => deleteRow(row.id)}
                          className="text-red-400 hover:text-red-600 transition-colors duration-150 p-1"
                        >
                          <Trash2 size={16} />
                        </button>
                      </>
                    )}
                    <button
                      onClick={() => toggleActions(row.id)}
                      className="text-slate-400 hover:text-slate-600 transition-colors duration-150 p-1"
                    >
                      <MoreVertical size={16} />
                    </button>
                  </div>
                </div>

                {/* Expanded Content */}
                {row.expanded && (
                  <div className="bg-slate-50 border-t border-slate-100">
                    <div className="px-4 py-3">
                      <pre className="text-xs text-slate-700 font-['Fira_Code'] leading-relaxed whitespace-pre-wrap">
                        {row.rawData}
                      </pre>
                    </div>
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}

export default DataPanel; 