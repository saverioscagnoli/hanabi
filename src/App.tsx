import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useTauriEvent } from "@util-hooks/use-tauri-event";
import { useState, useEffect } from "react";
import { cn } from "~/lib/utils";

function App() {
  const [time, setTime] = useState(new Date());
  const [cpuUsage, setCpuUsage] = useState(0);

  useEffect(() => {
    const interval = setInterval(() => {
      setTime(new Date());
    }, 1000);

    return () => {
      clearInterval(interval);
    };
  }, []);

  useTauriEvent<number>("cpu-usage", cpu => {
    setCpuUsage(cpu);
  });

  return (
    <div
      className={cn(
        "w-screen h-screen",
        "flex justify-center items-center gap-8",
        "text-white"
      )}>
      <h1>{time.toDateString()}</h1>
      <h1>{cpuUsage.toFixed(1)}%</h1>
      <button></button>
    </div>
  );
}

export { App };
