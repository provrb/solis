import "./App.css";
import Sidebar from "./sidebar";
import ConnectionPanel from "./ConnectionPanel";
import DataPanel from "./DataPanel";
import TitleBar from "./TitleBar";
import { useState, useEffect } from "react";
import { 
  MonitorUp, Move3d, Clock, Infinity, Flag, ToolCase, CarFront, 
  BatteryCharging, CircleAlert, User, Users, NotepadText, GalleryVerticalEnd 
} from "lucide-react";

function App() {
  const [activePanel, setActivePanel] = useState<string | null>(null);

  // List of all available panels from the sidebar
  const availablePanels = [
    "Connection",
    "Motion Data", 
    "Session Data",
    "Lap Data",
    "Events",
    "Car Setups",
    "Car Telemetry",
    "Car Status",
    "Car Damage",
    "Participants",
    "Lobby Info",
    "Final Classification",
    "Session History"
  ];

  // Randomly select 3 panels for the welcome screen cards
  const [randomCards, setRandomCards] = useState<string[]>([]);

  useEffect(() => {
    // Shuffle the available panels and take the first 3
    const shuffled = [...availablePanels].sort(() => 0.5 - Math.random());
    setRandomCards(shuffled.slice(0, 3));
  }, []);

  const handleSidebarItemClick = (itemLabel: string) => {
    setActivePanel(itemLabel);
  };

  // Function to get card data based on panel name
  const getCardData = (panelName: string) => {
    const cardData: { [key: string]: { icon: React.ReactNode, description: string } } = {
      "Connection": {
        icon: <MonitorUp size={24} />,
        description: "Establish UDP connections to racing simulators and configure packet formats for real-time data streaming."
      },
      "Motion Data": {
        icon: <Move3d size={24} />,
        description: "View detailed physics data for all cars being driven in the current session."
      },
      "Session Data": {
        icon: <Clock size={24} />,
        description: "View comprehensive session information including marshal zones, weather conditions, and track statistics."
      },
      "Lap Data": {
        icon: <Infinity size={24} />,
        description: "Detailed lap-by-lap analysis for all vehicles being driven in the current session including sector times, driver status, and penalties."
      },
      "Events": {
        icon: <Flag size={24} />,
        description: "Events and incidents that happen during the course of a session."
      },
      "Car Setups": {
        icon: <ToolCase size={24} />,
        description: "Comprehensize car setups of each vehicle in the session including aerodynamics, tires, and brakes."
      },
      "Car Telemetry": {
        icon: <CarFront size={24} />,
        description: "Monitor real-time vehicle data including speed, RPM, fuel levels, and engine performance metrics."
      },
      "Car Status": {
        icon: <BatteryCharging size={24} />,
        description: "Check the status of indicators like vehicle traction status, brake bias, and fuel capacity."
      },
      "Car Damage": {
        icon: <CircleAlert size={24} />,
        description: "Monitor every vehicle's damage levels, component wear, and faults throughout the session."
      },
      "Participants": {
        icon: <User size={24} />,
        description: "View driver information and participant details for the current racing session."
      },
      "Lobby Info": {
        icon: <Users size={24} />,
        description: "View details of players currently in a multiplayer lobby like their selected car or ready status."
      },
      "Final Classification": {
        icon: <NotepadText size={24} />,
        description: "View final race results, driver standings, and best lap times after session completion."
      },
      "Session History": {
        icon: <GalleryVerticalEnd size={24} />,
        description: "Browse historic lap times and tyre usage for the session."
      }
    };
    return cardData[panelName] || cardData["Connection"];
  };

  const renderMainContent = () => {
    switch (activePanel) {
      case "Connection":
        return <ConnectionPanel />;
      case "Motion Data":
      case "Session Data":
      case "Lap Data":
      case "Events":
      case "Car Setups":
      case "Car Telemetry":
      case "Car Status":
      case "Car Damage":
      case "Participants":
      case "Lobby Info":
      case "Final Classification":
      case "Session History":
        return <DataPanel title={activePanel} />;
      default:
        return (
          <div className="flex flex-col items-center justify-center h-full px-6">
            <div className="text-center mb-12">
              <h1 className="text-2xl font-bold text-slate-800 font-montserrat mb-2">
                Welcome to Solis
              </h1>
              <p className="text-slate-600 font-montserrat">
                Select an option from the sidebar or below to get started
              </p>
            </div>
            
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6 max-w-4xl w-full">
              {randomCards.map((panelName, index) => {
                const cardData = getCardData(panelName);
                return (
                  <div 
                    key={index} 
                    className="bg-white rounded-lg shadow-sm border border-slate-200 p-6 cursor-pointer"
                    onClick={() => setActivePanel(panelName)}
                  >
                    <div className="flex items-center mb-4">
                      <div className="w-10 h-10 bg-slate-100 rounded-lg flex items-center justify-center mr-3">
                        <div className="text-slate-600">
                          {cardData.icon}
                        </div>
                      </div>
                      <h3 className="text-lg font-semibold text-slate-800 font-montserrat">{panelName}</h3>
                    </div>
                    <p className="text-slate-500 font-montserrat text-sm leading-relaxed">
                      {cardData.description}
                    </p>
                  </div>
                );
              })}
            </div>
          </div>
        );
    }
  };

  return (
    <div className="flex flex-col h-screen bg-slate-50">
      <TitleBar />
      <div className="flex flex-1 overflow-hidden">
        <Sidebar onItemClick={handleSidebarItemClick} />
        <div className="flex-1 overflow-hidden">
          {renderMainContent()}
        </div>
      </div>
    </div>
  );
}

export default App;
