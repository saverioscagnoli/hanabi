import { useState } from "react";
import { useTauriEvent } from "~/hooks";

const Sysinfo = () => {
  const [cpuUsage, setCpuUsage] = useState<number>(0);
  const [cpuTemp, setCpuTemp] = useState<number>(0);
  const [memUsage, setMemUsage] = useState<number>(0);

  useTauriEvent("cpu-usage", u => setCpuUsage(u));
  useTauriEvent("cpu-temp", t => setCpuTemp(t));
  useTauriEvent("mem-usage", u => setMemUsage(u));
};

export { Sysinfo };
