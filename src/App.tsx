import { invoke } from "@tauri-apps/api/core";
import { useTauriEvent } from "@util-hooks/use-tauri-event";
import { useState, useEffect } from "react";
import { cn } from "~/lib/utils";

function App() {
  const [time, setTime] = useState(new Date());
  const [cpuUsage, setCpuUsage] = useState(0);
  const [activeWindowTitle, setActiveWindowTitle] = useState<string>("");
  const [workspaces, setWorkspaces] = useState<number[]>([]);
  const [activeWorkspace, setActiveWorkspace] = useState<number>(1);
  const [isWorkspaceChanging, setIsWorkspaceChanging] = useState(false);

  useEffect(() => {
    const interval = setInterval(() => {
      setTime(new Date());
    }, 1000);

    invoke<number[]>("fetch_workspaces").then(w => {
      console.log(w);
      setWorkspaces(w);
    });

    return () => {
      clearInterval(interval);
    };
  }, []);

  useTauriEvent<number>("cpu-usage", cpu => {
    setCpuUsage(cpu);
  });

  useTauriEvent<string>("window-changed", title => {
    setActiveWindowTitle(title);
  });

  useTauriEvent<[number, number[]]>("workspace-changed", ([id, w]) => {
    setActiveWorkspace(id);
    setIsWorkspaceChanging(true);

    if (w.length !== workspaces.length) {
      setWorkspaces(w);
    }

    setTimeout(() => {
      setIsWorkspaceChanging(false);
    }, 500); // Duration of the animation
  });

  return (
    <div
      className={cn(
        "w-screen h-screen",
        "flex justify-between items-center",
        "text-white"
      )}>
      <div className={cn("flex items-center gap-4")}>
        {workspaces.map(id => (
          <p
            key={id}
            className={cn(
              id === activeWorkspace ? "text-red-500" : "",
              isWorkspaceChanging ? "workspace-change" : ""
            )}>
            {id}
          </p>
        ))}
      </div>
      <p className={cn("flex items-center")} style={{ minWidth: "200px" }}>
        {activeWindowTitle}
      </p>
      <div className={cn("flex justify-center items-center gap-4")}>
        <h1 style={{ minWidth: "150px" }}>{time.toDateString()}</h1>
        <h1 style={{ minWidth: "100px" }}>{cpuUsage.toFixed(1)}%</h1>
      </div>
    </div>
  );
}

export { App };
