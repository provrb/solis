import React, { useState, useRef, useEffect } from "react";
import { 
  Flag, 
  BatteryCharging, 
  CircleAlert,
  ToolCase, 
  Clock, 
  User, 
  CarFront,
  Move3d,
  NotepadText,
  Users,
  GalleryVerticalEnd,
  Infinity,
  MonitorUp
} from "lucide-react";


interface SidebarItem {
  icon: React.ReactNode;
  label: string;
  active?: boolean;
}

interface SidebarProps {
  onItemClick?: (itemLabel: string) => void;
}

function Sidebar({ onItemClick }: SidebarProps) {
  const [width, setWidth] = useState(256); // 256px = w-64
  const [isResizing, setIsResizing] = useState(false);
  const sidebarRef = useRef<HTMLDivElement>(null);
  
  const minWidth = 60; // Minimum width - just enough for icons
  const maxWidth = 384; // 1.5x the original 256px = 384px
  
  const sidebarItems: SidebarItem[] = [
    { icon: <MonitorUp size={20} />, label: "Connection" },
    { icon: <Move3d size={20} />, label: "Motion Data" },
    { icon: <Clock size={20} />, label: "Session Data" },
    { icon: <Infinity size={20} />, label: "Lap Data" },
    { icon: <Flag size={20} />, label: "Events" },
    { icon: <ToolCase size={20} />, label: "Car Setups" },
    { icon: <CarFront size={20} />, label: "Car Telemetry" },
    { icon: <BatteryCharging size={20} />, label: "Car Status" },
    { icon: <CircleAlert size={20} />, label: "Car Damage" }, 
    { icon: <User size={20} />, label: "Participants" },
    { icon: <Users size={20} />, label: "Lobby Info" },
    { icon: <NotepadText size={20} />, label: "Final Classification" },
    { icon: <GalleryVerticalEnd size={20} />, label: "Session History" },
  ];

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);
  };

  const handleMouseMove = (e: MouseEvent) => {
    if (!isResizing) return;
    
    const newWidth = e.clientX;
    if (newWidth >= minWidth && newWidth <= maxWidth) {
      setWidth(newWidth);
    }
  };

  const handleMouseUp = () => {
    setIsResizing(false);
  };

  const handleItemClick = (itemLabel: string) => {
    if (onItemClick) {
      onItemClick(itemLabel);
    }
  };

  useEffect(() => {
    if (isResizing) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
      
      return () => {
        document.removeEventListener('mousemove', handleMouseMove);
        document.removeEventListener('mouseup', handleMouseUp);
      };
    }
  }, [isResizing]);

  return (
    <div 
      ref={sidebarRef}
      className="h-screen bg-white border-r border-slate-200 flex flex-col shadow-sm font-montserrat relative"
      style={{ width: `${width}px` }}
    >
      {/* Navigation */}
      <nav className="flex-1 px-4 py-4 space-y-2 overflow-hidden">
        {/* Group 1: Connection */}
        {sidebarItems.slice(0, 1).map((item, index) => (
          <button
            key={index}
            onClick={() => handleItemClick(item.label)}
            className={`w-full flex items-center space-x-3 pl-2 pr-4 py-2.5 rounded-lg transition-all duration-200 group min-w-0`}
          >
            <div className={`transition-colors duration-200 flex-shrink-0 ${
              item.active ? "text-blue-600" : "text-slate-400 group-hover:text-slate-600"
            }`}>
              {item.icon}
            </div>
            <span className={`font-medium transition-colors duration-200 text-sm truncate ${
              item.active ? "text-blue-600" : "text-slate-500 group-hover:text-slate-700"
            }`}>
              {item.label}
            </span>
          </button>
        ))}
        
        {/* Separator */}
        <div className="h-px bg-slate-200 my-4"></div>
        
        {/* Group 2: Motion Data, Session Data, Lap Data */}
        {sidebarItems.slice(1, 4).map((item, index) => (
          <button
            key={index + 1}
            onClick={() => handleItemClick(item.label)}
            className={`w-full flex items-center space-x-3 pl-2 pr-4 py-2.5 rounded-lg transition-all duration-200 group min-w-0`}
          >
            <div className={`transition-colors duration-200 flex-shrink-0 ${
              item.active ? "text-blue-600" : "text-slate-400 group-hover:text-slate-600"
            }`}>
              {item.icon}
            </div>
            <span className={`font-medium transition-colors duration-200 text-sm truncate ${
              item.active ? "text-blue-600" : "text-slate-500 group-hover:text-slate-700"
            }`}>
              {item.label}
            </span>
          </button>
        ))}
        
        {/* Separator */}
        <div className="h-px bg-slate-200 my-4"></div>
        
        {/* Group 3: Events */}
        {sidebarItems.slice(4, 5).map((item, index) => (
          <button
            key={index + 4}
            onClick={() => handleItemClick(item.label)}
            className={`w-full flex items-center space-x-3 pl-2 pr-4 py-2.5 rounded-lg transition-all duration-200 group min-w-0`}
          >
            <div className={`transition-colors duration-200 flex-shrink-0 ${
              item.active ? "text-blue-600" : "text-slate-400 group-hover:text-slate-600"
            }`}>
              {item.icon}
            </div>
            <span className={`font-medium transition-colors duration-200 text-sm truncate ${
              item.active ? "text-blue-600" : "text-slate-500 group-hover:text-slate-700"
            }`}>
              {item.label}
            </span>
          </button>
        ))}
        
        {/* Separator */}
        <div className="h-px bg-slate-200 my-4"></div>
        
        {/* Group 4: Car Setups, Car Telemetry, Car Status, Car Damage */}
        {sidebarItems.slice(5, 9).map((item, index) => (
          <button
            key={index + 5}
            onClick={() => handleItemClick(item.label)}
            className={`w-full flex items-center space-x-3 pl-2 pr-4 py-2.5 rounded-lg transition-all duration-200 group min-w-0`}
          >
            <div className={`transition-colors duration-200 flex-shrink-0 ${
              item.active ? "text-blue-600" : "text-slate-400 group-hover:text-slate-600"
            }`}>
              {item.icon}
            </div>
            <span className={`font-medium transition-colors duration-200 text-sm truncate ${
              item.active ? "text-blue-600" : "text-slate-500 group-hover:text-slate-700"
            }`}>
              {item.label}
            </span>
          </button>
        ))}
        
        {/* Separator */}
        <div className="h-px bg-slate-200 my-4"></div>
        
        {/* Group 5: Participants, Lobby Info, Final Classification, Session History */}
        {sidebarItems.slice(9, 13).map((item, index) => (
          <button
            key={index + 9}
            onClick={() => handleItemClick(item.label)}
            className={`w-full flex items-center space-x-3 pl-2 pr-4 py-2.5 rounded-lg transition-all duration-200 group min-w-0`}
          >
            <div className={`transition-colors duration-200 flex-shrink-0 ${
              item.active ? "text-blue-600" : "text-slate-400 group-hover:text-slate-600"
            }`}>
              {item.icon}
            </div>
            <span className={`font-medium transition-colors duration-200 text-sm truncate ${
              item.active ? "text-blue-600" : "text-slate-500 group-hover:text-slate-700"
            }`}>
              {item.label}
            </span>
          </button>
        ))}
      </nav>
      
      {/* Resize Handle */}
      <div
        className="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-400 transition-colors duration-200"
        onMouseDown={handleMouseDown}
        style={{ cursor: isResizing ? 'col-resize' : 'default' }}
      />
    </div>
  );
}

export default Sidebar;