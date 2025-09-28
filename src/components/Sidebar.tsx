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
  MonitorUp,
  Headset,
} from "lucide-react";

interface SidebarItem {
  icon: React.ReactNode;
  label: string;
  active?: boolean;
}

interface SidebarProps {
  onItemClick?: (itemLabel: string) => void;
  activePanel?: string | null;
}

function Sidebar({ onItemClick, activePanel }: SidebarProps) {
  const [width] = useState(256); // 256px = w-64
  const sidebarRef = useRef<HTMLDivElement>(null);
  const navRef = useRef<HTMLDivElement>(null);
  const setCanScrollUp = useState(false)[1];
  const setCanScrollDown = useState(false)[1];

  const sidebarItems: SidebarItem[] = [
    { icon: <MonitorUp size={20} />, label: "Connection" },
    { icon: <Headset size={20} />, label: "Audio" },
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
  ].map((item) => ({ ...item, active: activePanel === item.label }));

  const handleItemClick = (itemLabel: string) => {
    if (onItemClick) {
      onItemClick(itemLabel);
    }
  };

  // Scroll handler
  const handleScroll = () => {
    const nav = navRef.current;
    if (!nav) return;
    setCanScrollUp(nav.scrollTop > 0);
    setCanScrollDown(nav.scrollTop + nav.clientHeight < nav.scrollHeight);
  };

  useEffect(() => {
    handleScroll();
  }, []);

  // Attach scroll event
  useEffect(() => {
    const nav = navRef.current;
    if (!nav) return;
    nav.addEventListener("scroll", handleScroll);
    handleScroll();
    return () => nav.removeEventListener("scroll", handleScroll);
  }, []);

  return (
    <div
      ref={sidebarRef}
      className="h-screen bg-white border-r border-slate-200 flex flex-col shadow-sm font-montserrat relative"
      style={{ width: `${width}px` }}
    >
      {/* Navigation */}
      <nav
        ref={navRef}
        className="flex-1 px-4 pt-4 pb-12 space-y-2 overflow-y-auto scrollbar-hide relative"
        style={{ scrollBehavior: "smooth" }}
      >
        {/* Group 1: Connection */}
        {sidebarItems.slice(0, 2).map((item, index) => (
          <button
            key={index}
            onClick={() => handleItemClick(item.label)}
            className={`w-full flex items-center space-x-3 pl-2 pr-4 py-2.5 rounded-lg transition-all duration-200 group min-w-0 ${item.active ? "text-slate-900" : ""}`}
          >
            <div
              className={`transition-colors duration-200 flex-shrink-0 ${item.active ? "text-slate-900" : "text-slate-400 group-hover:text-slate-600"}`}
            >
              {item.icon}
            </div>
            <span
              className={`font-medium transition-colors duration-200 text-sm truncate ${item.active ? "text-slate-900" : "text-slate-500 group-hover:text-slate-700"}`}
            >
              {item.label}
            </span>
          </button>
        ))}

        {/* Separator */}
        <div className="h-px bg-slate-200 my-4"></div>

        {/* Group 2: Motion Data, Session Data, Lap Data */}
        {sidebarItems.slice(2, 5).map((item, index) => (
          <button
            key={index + 1}
            onClick={() => handleItemClick(item.label)}
            className={`w-full flex items-center space-x-3 pl-2 pr-4 py-2.5 rounded-lg transition-all duration-200 group min-w-0 ${item.active ? "text-slate-900" : ""}`}
          >
            <div
              className={`transition-colors duration-200 flex-shrink-0 ${item.active ? "text-slate-900" : "text-slate-400 group-hover:text-slate-600"}`}
            >
              {item.icon}
            </div>
            <span
              className={`font-medium transition-colors duration-200 text-sm truncate ${item.active ? "text-slate-900" : "text-slate-500 group-hover:text-slate-700"}`}
            >
              {item.label}
            </span>
          </button>
        ))}

        {/* Separator */}
        <div className="h-px bg-slate-200 my-4"></div>

        {/* Group 3: Events */}
        {sidebarItems.slice(5, 6).map((item, index) => (
          <button
            key={index + 4}
            onClick={() => handleItemClick(item.label)}
            className={`w-full flex items-center space-x-3 pl-2 pr-4 py-2.5 rounded-lg transition-all duration-200 group min-w-0 ${item.active ? "text-slate-900" : ""}`}
          >
            <div
              className={`transition-colors duration-200 flex-shrink-0 ${item.active ? "text-slate-900" : "text-slate-400 group-hover:text-slate-600"}`}
            >
              {item.icon}
            </div>
            <span
              className={`font-medium transition-colors duration-200 text-sm truncate ${item.active ? "text-slate-900" : "text-slate-500 group-hover:text-slate-700"}`}
            >
              {item.label}
            </span>
          </button>
        ))}

        {/* Separator */}
        <div className="h-px bg-slate-200 my-4"></div>

        {/* Group 4: Car Setups, Car Telemetry, Car Status, Car Damage */}
        {sidebarItems.slice(6, 10).map((item, index) => (
          <button
            key={index + 5}
            onClick={() => handleItemClick(item.label)}
            className={`w-full flex items-center space-x-3 pl-2 pr-4 py-2.5 rounded-lg transition-all duration-200 group min-w-0 ${item.active ? "text-slate-900" : ""}`}
          >
            <div
              className={`transition-colors duration-200 flex-shrink-0 ${item.active ? "text-slate-900" : "text-slate-400 group-hover:text-slate-600"}`}
            >
              {item.icon}
            </div>
            <span
              className={`font-medium transition-colors duration-200 text-sm truncate ${item.active ? "text-slate-900" : "text-slate-500 group-hover:text-slate-700"}`}
            >
              {item.label}
            </span>
          </button>
        ))}

        {/* Separator */}
        <div className="h-px bg-slate-200 my-4"></div>

        {/* Group 5: Participants, Lobby Info, Final Classification, Session History */}
        {sidebarItems.slice(10, 14).map((item, index) => (
          <button
            key={index + 9}
            onClick={() => handleItemClick(item.label)}
            className={`w-full flex items-center space-x-3 pl-2 pr-4 py-2.5 rounded-lg transition-all duration-200 group min-w-0 ${item.active ? "text-slate-900" : ""}`}
          >
            <div
              className={`transition-colors duration-200 flex-shrink-0 ${item.active ? "text-slate-900" : "text-slate-400 group-hover:text-slate-600"}`}
            >
              {item.icon}
            </div>
            <span
              className={`font-medium transition-colors duration-200 text-sm truncate ${item.active ? "text-slate-900" : "text-slate-500 group-hover:text-slate-700"}`}
            >
              {item.label}
            </span>
          </button>
        ))}
      </nav>
    </div>
  );
}

export default Sidebar;
