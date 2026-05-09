import { writable } from "svelte/store";
import type { ClientId } from "../clients/config";
import { ALL_CLIENTS } from "../clients/config";

const STORAGE_KEY = "tokscale-monitor-settings";

export interface ProviderSettings {
  enabled: Record<string, boolean>;
}

function createDefaultSettings(): ProviderSettings {
  const enabled: Record<string, boolean> = {};
  for (const client of ALL_CLIENTS) {
    enabled[client] = true;
  }
  return { enabled };
}

function loadSettings(): ProviderSettings {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw) {
      const parsed = JSON.parse(raw) as ProviderSettings;
      // Merge with defaults to handle new providers
      const defaults = createDefaultSettings();
      return {
        enabled: { ...defaults.enabled, ...parsed.enabled },
      };
    }
  } catch {
    // ignore parse errors
  }
  return createDefaultSettings();
}

function saveSettings(settings: ProviderSettings) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
}

function createSettingsStore() {
  const initial = loadSettings();
  const { subscribe, set, update } = writable<ProviderSettings>(initial);

  return {
    subscribe,
    toggleProvider: (client: ClientId) => {
      update((s) => {
        const next = {
          ...s,
          enabled: { ...s.enabled, [client]: !s.enabled[client] },
        };
        saveSettings(next);
        return next;
      });
    },
    reset: () => {
      const defaults = createDefaultSettings();
      saveSettings(defaults);
      set(defaults);
    },
  };
}

export const settingsStore = createSettingsStore();
