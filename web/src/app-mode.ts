export const appMode = import.meta.env.VITE_APP_MODE ?? "default";

export const isMockMode = appMode === "mock";
