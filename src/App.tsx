import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [msg, setMsg] = useState("");
  const [isRunning, setIsRunning] = useState(false);
  const [version, setVersion] = useState("");

  async function submit() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    if (isRunning) {
      await stopService();
    } else {
      await startService();
    }
    setIsRunning(!isRunning);
  }

  // 启动 SDK
  async function startService() {
    const code = await invoke("start_tun_with_config_file", { config: "encrypted_config.txt" });
    if(code === 0){
      setMsg(`Started successfully`);
    }else{
      setMsg(`StartTun2R result: ${code}`);
    }
  }

  // 停止
  async function stopService() {
    const code = await invoke("stop_tun");
    if(code === 0){
      setMsg(`Stopped successfully`);
    }else{
      setMsg(`StopTun2R result: ${code}`);
    }
  }

  // 获取 SDK 版本
  async function getVersion() {
    const version = await invoke("sdk_version");
    setVersion(`SDK Version: ${version}`);
  }

  return (
    <main className="container">
      <h1>Welcome to Lingti Dev Kit</h1>
      <p>{version}</p>

      <div className="row" onClick={() => getVersion()}>
        <img src="https://res.lingti666.com/img/icon/pc/default.png" style={{ width: "230px" }} alt="Lingti Dev Kit logo" />
      </div>
      <div className="row" style={{ marginTop: "30px" }}>
        <button type="submit" onClick={() => {
          submit();
        }}>{isRunning ? "Stop" : "Start"}</button>
      </div>
      <p>{msg}</p>
    </main>
  );
}

export default App;
