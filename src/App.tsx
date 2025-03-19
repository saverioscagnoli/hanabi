import { invoke } from "@tauri-apps/api/core";
import { useTauriEvent } from "@util-hooks/use-tauri-event";
import { useState, useEffect } from "react";
import { cn } from "~/lib/utils";
import { Workspaces } from "~/components/workspaces";
import { GoCpu } from "react-icons/go";

function App() {
  const [time, setTime] = useState(new Date());
  const [cpuUsage, setCpuUsage] = useState(0);
  const [memUsage, setMemUsage] = useState(0);
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

  useTauriEvent<number>("cpu-usage", cpu => {
    setCpuUsage(cpu);
  });

  useTauriEvent<number>("mem-usage", mem => {
    setMemUsage(mem);
  });

  useTauriEvent<string>("window-changed", title => {
    setActiveWindowTitle(title);
  });

  return (
    <div
      className={cn(
        "w-screen h-screen",
        "flex justify-between items-center",
        "text-white",
        "select-none",
        "cursor-default"
      )}>
      <Workspaces />
      <div className={cn("min-w-1/3 flex justify-center items-center")}>
        <p className={cn("flex items-center", "shrink-0")}>
          {activeWindowTitle}
        </p>
      </div>
      <div className={cn("min-w-1/3 flex justify-center items-center gap-4")}>
        <h1 style={{ minWidth: "150px" }}>{time.getDay()}/{time.getMonth()}/{time.getFullYear()} {time.getHours()}:{time.getMinutes()}</h1>
        <span className={cn("flex items-center gap-2")}>
          <GoCpu />
        <h1 style={{ minWidth: "100px" }}>{cpuUsage.toFixed(1)} %</h1>

        </span>
        <h1>mem: {memUsage.toFixed(1)} %</h1>
      </div>
    </div>
  );
}

export { App };