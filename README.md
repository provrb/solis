<div align="center" style="display: flex; align-items: center; justify-content: center; gap: 20px;">
  <img src="src-tauri/icons/solis-128x128-2.png" style="filter: brightness(0) invert(1); height: 80px;">
  <span style="font-weight: 600; font-size: 80px; line-height: 80px; margin: 0; text-decoration: none;">Solis</span>
</div>

<div style="max-width: 700px; margin: 20px auto; text-align: center;">
  <p><em>An AI-driven race engineer that interacts through natural conversation, analyzing real-time telemetry data to deliver precise insights and optimize race strategies for smarter on-track decisions.</em></p>

  <img alt="last-commit" src="https://img.shields.io/github/last-commit/provrb/solis?style=flat&logo=git&logoColor=white&color=0080ff" style="margin-right: 6px;">
  <img alt="repo-top-language" src="https://img.shields.io/github/languages/top/provrb/solis?style=flat&color=0080ff" style="margin-right: 6px;">
  <img alt="repo-language-count" src="https://img.shields.io/github/languages/count/provrb/solis?style=flat&color=0080ff">
</div>

![](/docs/home.png)

<details>
   <summary><b>Click to view app screenshots!</b></summary>

   ![](/docs/home.png)
   <p align="center"><em>Home Screen</em></p>

   ![](/docs/connect.png)
   <p align="center"><em>Connection Screen</em></p>

   ![](/docs/car_damage.png)
   <p align="center"><em>Car Damage Telemtry Updates</em></p>

   ![](/docs/motion_data.png)
   <p align="center"><em>Motion Data Telemtry Updates</em></p>

   ![](/docs/session_data.png)
   <p align="center"><em>Session Data Telemtry Updates</em></p>  
</details>

## Table of Contents
- [Overview](#overview)
- [Features](#features)
- [License](#license)
- [Contributing](#contributing)
- [Roadmap](#roadmap)

## Overview
**Solis is what every race enthusiast has wanted from CodeMaster & EA for years.**

**A real-time, AI race engineer** built in Rust. Designed for performance, clarity, and the potential to port the backend logic to embedded systems, Solis decodes and interprets F1 telemetry data via UDP, enabling intelligent feedback and strategic insight for drivers, developers, and enthusiasts prompted by a simple voice request.

Solis is not just a telemetry parser, but a modular system designed to learn, analyze, and communicate racing insights, bringing professional-grade race strategy to the track, simulator, or garage- breaking the bottleneck between the pit wall and the track.

## Features
- **📡 Real-Time Telemetry Ingestion** Parses high-frequency telemetry packets over UDP (F1 22+), including motion, lap, session, event, car status, damage, and more.

- **🧠 Intelligent Packet Decoding**  Implements a clean, strongly-typed, extensible decoding layer in Rust for all known F1 telemetry formats.

- **📊 Visual Debug UI**  A modern React + Tauri frontend to inspect live data, ideal for debugging, development, and analysis.

- **🛠️ Embedded & Offline-first**  Designed to run in headless mode, suitable for deployment in simulators, dashboards, or embedded race systems.

- **📁 Modular Architecture**  Each packet type has a clear, readable implementation transformed to Rust from the original C CodeMasters implementation.

- **🚀 Fast & Safe**  Built with Rust for guaranteed memory safety and consistent performance under the extreme demands of race conditions and high-frequency telemetry. Solis runs under 5% CPU and uses less than 500 MB of RAM, even at sustained 1000Hz data rates.


## Installation

### Installer

1. Head to the GitHub releases page [here](https://github.com/provrb/solis/releases)
2. Download the latest release for your operating system
3. Extract and run the installer.

### Command-Line

1. **Install Rust**  
   Download and install from [rust-lang.org](https://www.rust-lang.org/tools/install).

1. **Install Node.js & npm**  
   Download and install from [nodejs.org](https://nodejs.org/en/download).

3. **Install Tauri**

   ```sh
   cargo install tauri-cli --version 1.6.5
   ```

4. **Clone the repository**

   ```sh
   git clone https://github.com/provrb/solis.git
   cd solis
   ```

5. **Run the Project (Debugging/Development)**
   ```sh
   npm run tauri dev
   ```

5. **Build the project (Installer's, EXE's, AppImage, etc)**

   ```sh
   npm install
   npm run tauri build
   ```

6. **Find the application**:

   - The file built will be located in: `src-tauri/target/release`
   - Installers will be located in: `src-tauri/target/release/bundle`

## Usage
### Formula 1
1. Enable UDP telemetry
    1. Open Game Options > Settings > Telemetry Settings
    2. Set `UDP Telemtry` to `on`
    3. Set `UDP Broadcast Mode` to `off`
    4. Set `UDP IP Address` to the IP address of the device running Solis. Leave as `127.0.0.1` if Solis is running on the current machine.
    - Leave `Port`, `UDP Send Rate`, and `UDP format` as the default value
2. Create UDP socket in Solis
    1. Navigate to the `Connection` page in Solis
    2. Select the IP address you chose for `UDP IP Address` in your F1 settings. 
    3. Select the Port you chose for `Port` in your F1 settings. 
    4. **Select the UDP format you chose for `UDP format` in F1.**/
3. Play a match!
    - All information will be shown in their respective panel.
        - e.g: motion data will be in the `Motion Data` panel.

## Roadmap
Solis aims to redefine the race engineer experience with embedded AI, uncompromising performance, and transparency. Solis is the engineer who never sleeps.

### Plans for the future include:
- Implement speech-to-text for voice input to include in the request to Solis
- Implement local AI model to generate responses (Not GPT wrapper)
- Implement text-to-speech for Solis to respond with a voice
- Add support for F1 23, 24, 25

## License
Solis is licensed under the GNU General Public License v3.0. See [LICENSE](/LICENSE.md) for more details.