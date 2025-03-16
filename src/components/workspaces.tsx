import { invoke } from "@tauri-apps/api/core";
import { useTauriEvent } from "@util-hooks/use-tauri-event";
import { useEffect, useState } from "react";
import { cn } from "~/lib/utils";

const Workspaces = () => {
  const [workspaces, setWorkspaces] = useState<number[]>([]);
  const [activeWorkspace, setActiveWorkspace] = useState<number>(1);

  useEffect(() => {
    invoke<number[]>("fetch_workspaces").then(setWorkspaces);
  }, []);

  useTauriEvent<[number, number[]]>("workspace-changed", ([wID, ws]) => {
    setActiveWorkspace(wID);

    if (ws.length !== workspaces.length) {
      setWorkspaces(ws);
    }
  });

  return (
    <div className={cn("flex items-center gap-4")}>
      {workspaces.sort().map(id => (
        <p
          key={id}
          className={cn(id === activeWorkspace ? "text-red-500" : "")}>
          {id}
        </p>
      ))}
    </div>
  );
};

export { Workspaces };