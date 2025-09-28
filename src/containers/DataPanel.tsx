import React, { useEffect, useState } from "react";
import {
  ChevronDown,
  ChevronRight,
  MoreVertical,
  Trash2,
  ChevronUp,
} from "lucide-react";
import { listen } from "@tauri-apps/api/event";
import Header from "../components/Header";

interface DataRow {
  id: string;
  timestamp: string;
  packetId: string;
  title: string;
  rowTitle: string;
  expanded: boolean;
  showActions: boolean;
  rawData: string;
}

interface DataPanelProps {
  title: string;
  dataRowsByTitle: Record<string, DataRow[]>;
  setDataRowsByTitle: React.Dispatch<
    React.SetStateAction<Record<string, DataRow[]>>
  >;
}

// Function to get subtitle based on panel type
const getSubtitle = (title: string): string => {
  switch (title) {
    case "Audio":
      return "Configure audio input/output settings";
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

const MAX_ROWS = 250;

export default function DataPanel({
  title,
  dataRowsByTitle,
  setDataRowsByTitle,
}: DataPanelProps) {
  // Track expanded rows by their unique id
  const [expandedRows, setExpandedRows] = useState<Set<string>>(new Set());
  // Track which row's actions are open by their unique id
  const [actionsOpen, setActionsOpen] = useState<Set<string>>(new Set());

  // Only show rows for the current panel's title (using rowTitle)
  // Prevent clearing dataRowsByTitle when switching to the connection panel
  const dataRowsForPanel =
    title === "Connection Panel" ? [] : dataRowsByTitle[title] || [];

  useEffect(() => {
    const unlistenPromise = listen("createDataRowBatch", (event: any) => {
      setDataRowsByTitle((prev) => {
        // Clone previous state
        const newState = { ...prev };
        for (const row of event.payload) {
          console.log(row);

          // Ensure each row has a unique, stable id
          if (!row.id) {
            row.id =
              Date.now().toString() + Math.random().toString(36).substr(2, 9);
          }

          const rowTitle = row.title || "Unknown";
          if (!newState[rowTitle]) newState[rowTitle] = [];
          // Prevent duplicate ids in the list
          if (!newState[rowTitle].some((r) => r.id === row.id)) {
            newState[rowTitle] = [row, ...newState[rowTitle]];
            if (newState[rowTitle].length > MAX_ROWS) {
              newState[rowTitle] = newState[rowTitle].slice(0, MAX_ROWS);
            }
          }
        }
        return newState;
      });
    });
    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  const toggleExpanded = (id: string) => {
    setExpandedRows((prev) => {
      const newSet = new Set(prev);
      if (newSet.has(id)) {
        newSet.delete(id);
      } else {
        newSet.add(id);
      }
      return newSet;
    });
  };

  const toggleActions = (id: string) => {
    setActionsOpen((prev) => {
      const newSet = new Set(prev);
      if (newSet.has(id)) {
        newSet.delete(id);
      } else {
        newSet.add(id);
      }
      return newSet;
    });
  };

  const deleteRow = (id: string) => {
    setDataRowsByTitle((prev) => {
      // Defensive: copy the array before mutating
      const rows = prev[title] ? [...prev[title]] : [];
      const idx = rows.findIndex((row) => row.id === id);
      if (idx !== -1) {
        rows.splice(idx, 1);
      }
      return { ...prev, [title]: rows };
    });
    setActionsOpen((prev) => {
      const newObj = new Set(prev);
      newObj.delete(id);
      return newObj;
    });
  };

  const moveRow = (id: string, direction: "up" | "down") => {
    setDataRowsByTitle((prev) => {
      const rows = prev[title] ? [...prev[title]] : [];
      const idx = rows.findIndex((row) => row.id === id);
      if (idx === -1) return prev;
      if (direction === "up" && idx > 0) {
        [rows[idx - 1], rows[idx]] = [rows[idx], rows[idx - 1]];
      } else if (direction === "down" && idx < rows.length - 1) {
        [rows[idx], rows[idx + 1]] = [rows[idx + 1], rows[idx]];
      }
      return { ...prev, [title]: rows };
    });
  };

  return (
    <div className="h-full bg-slate-50 overflow-y-auto">
      {/* Header */}
      <div>
        <Header title={title} subtitle={getSubtitle(title)} />
      </div>

      {/* Data List */}
      <div className="px-6 flex-1 overflow-hidden">
        <div className="bg-white rounded-lg border border-slate-200 overflow-hidden">
          <div className="overflow-y-auto max-h-[calc(100vh-200px)]">
            {dataRowsForPanel.map((row) => {
              return (
                <div
                  key={row.id}
                  className="border-b border-slate-100 last:border-b-0"
                >
                  {/* Main Row */}
                  <div className="flex items-center px-4 py-3 transition-colors duration-150">
                    {/* Dropdown Arrow */}
                    <button
                      onClick={() => toggleExpanded(row.id)}
                      className="mr-3 text-slate-400 hover:text-slate-600 transition-colors duration-150"
                    >
                      {expandedRows.has(row.id) ? (
                        <ChevronDown size={16} />
                      ) : (
                        <ChevronRight size={16} />
                      )}
                    </button>
                    {/* Timestamp */}
                    <div className="text-sm text-slate-600 font-montserrat mr-4">
                      {row.timestamp} s
                    </div>
                    {/* Packet ID */}
                    <div className="text-sm text-slate-500 font-montserrat mr-4">
                      {row.rowTitle}
                    </div>
                    {/* Spacer */}
                    <div className="flex-1"></div>
                    {/* Actions */}
                    <div className="flex items-center space-x-2">
                      {actionsOpen.has(row.id) && (
                        <>
                          <button
                            onClick={() => moveRow(row.id, "up")}
                            className="text-slate-400 hover:text-slate-600 transition-colors duration-150 p-1"
                          >
                            <ChevronUp size={16} />
                          </button>
                          <button
                            onClick={() => moveRow(row.id, "down")}
                            className="text-slate-400 hover:text-slate-600 transition-colors duration-150 p-1"
                          >
                            <ChevronDown size={16} />
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
                  {expandedRows.has(row.id) && (
                    <div className="bg-slate-50 border-t border-slate-100">
                      <div
                        className="px-4 py-3 w-full max-w-full break-words whitespace-pre-wrap overflow-y-auto"
                        style={{ maxHeight: "300px" }}
                      >
                        <span className="text-xs text-slate-700 font-['Fira_Code'] leading-relaxed">
                          {row.rawData}
                        </span>
                      </div>
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </div>
      </div>
    </div>
  );
}
