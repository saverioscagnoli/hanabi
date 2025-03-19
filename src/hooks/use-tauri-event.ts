import { buildMappedTauriEventHook } from "@util-hooks/use-tauri-event";

type EventMap = {
  "cpu-usage": number;
  "cpu-temp": number;
  "mem-usage": number;
};

const useTauriEvent = buildMappedTauriEventHook<EventMap>();

export { useTauriEvent };
