export interface WindowConfig {
  label: string;
  hours: number;
  limit: number;
}

export interface PlanConfig {
  targetModels: string[];
  windows: WindowConfig[];
}

export const GO_PLAN_CONFIG: PlanConfig = {
  targetModels: [
    "GLM-5.1",
    "GLM-5",
    "Kimi-K2.5",
    "Kimi-K2.6",
    "MiMo-V2.5-Pro",
    "MiMo-V2.5",
    "Qwen-3.5-Plus",
    "Qwen-3.6-Plus",
    "MiniMax-M2.5",
    "MiniMax-M2.7",
    "DeepSeek-V4-Pro",
    "DeepSeek-V4-Flash",
  ],
  windows: [
    { label: "5h", hours: 5, limit: 12.0 },
    { label: "Weekly", hours: 7 * 24, limit: 30.0 },
    { label: "Monthly", hours: 30 * 24, limit: 60.0 },
  ],
};
