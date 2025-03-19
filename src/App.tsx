import { invoke } from "@tauri-apps/api/core";
import { useTauriEvent } from "@util-hooks/use-tauri-event";
import { useState, useEffect } from "react";
import { cn } from "~/lib/utils";
import { Workspaces } from "~/components/workspaces";

function App() {
  const [time, setTime] = useState(new Date());
  const [cpuUsage, setCpuUsage] = useState(0);
  const [activeWindowTitle, setActiveWindowTitle] = useState<string>("");

  useEffect(() => {
    const interval = setInterval(() => {
      setTime(new Date());
    }, 1000);

    invoke("spawn_metrics_threads");
    invoke("init_compositor_events");

    invoke<string | null>("fetch_active_window_title").then(
      title => title && setActiveWindowTitle(title)
    );

    return () => {
      clearInterval(interval);
    };
  }, []);

  useTauriEvent<number>("cpu-temp", cpu => {
    setCpuUsage(cpu);
  });

  useTauriEvent<string>("window-changed", title => {
    setActiveWindowTitle(title);
  });

  return (
    <div
      className={cn(
        "w-screen h-screen",
        "flex justify-between items-center",
        "text-white"
      )}>
      <Workspaces />
      <p className={cn("flex items-center")} style={{ minWidth: "200px" }}>
        {activeWindowTitle}
      </p>
      <div className={cn("flex justify-center items-center gap-4")}>
        <h1 style={{ minWidth: "150px" }}>{time.toDateString()}</h1>
        <h1 style={{ minWidth: "100px" }}>{cpuUsage.toFixed(1)} C</h1>
      </div>
    </div>
  );
}

export { App };
