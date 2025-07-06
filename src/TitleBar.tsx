import React from "react";
import { Minus, Square, X } from "lucide-react";
import { getCurrentWebviewWindow, WebviewWindow } from "@tauri-apps/api/webviewWindow";

function TitleBar() {
  const webviewWindow: WebviewWindow = getCurrentWebviewWindow();

  const handleMinimize = () => {
    webviewWindow.minimize();
  };

  const handleMaximize = async () => {
    if (await webviewWindow.isMaximized()) {
      webviewWindow.unmaximize();
    } else {
      webviewWindow.maximize();
    }
  };

  const handleClose = () => {
    webviewWindow.close();
  };

  return (
    <div className="h-8 bg-white border-b border-slate-200 flex items-center justify-between px-4 select-none relative" style={{ WebkitAppRegion: 'drag' } as React.CSSProperties}>
      {/* App Title - Centered */}
      <div className="absolute inset-0 flex items-center justify-center pointer-events-none">
        <h1 className="text-sm font-semibold text-slate-200 font-montserrat">Solis</h1>
      </div>

      {/* Window Controls */}
      <div className="flex items-center space-x-1 ml-auto" style={{ WebkitAppRegion: 'no-drag' } as React.CSSProperties}>
        <button
          onClick={handleMinimize}
          className="w-8 h-8 flex items-center justify-center text-slate-600 hover:bg-slate-100 transition-colors duration-150"
        >
          <Minus size={14} />
        </button>
        <button
          onClick={handleMaximize}
          className="w-8 h-8 flex items-center justify-center text-slate-600 hover:bg-slate-100 transition-colors duration-150"
        >
          <Square size={14} />
        </button>
        <button
          onClick={handleClose}
          className="w-8 h-8 flex items-center justify-center text-slate-600 hover:bg-red-500 hover:text-white transition-colors duration-150"
        >
          <X size={14} />
        </button>
      </div>
    </div>
  );
}

export default TitleBar; 