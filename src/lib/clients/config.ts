import type { PlanConfig } from "../planConfig";

export type ClientId =
  | "opencode"
  | "claude"
  | "codex"
  | "cursor"
  | "windsurf"
  | "qwen";

export type DataSourceType = "sqlite" | "json" | "jsonl";

export interface ClientConfig {
  id: ClientId;
  label: string;
  dataSource: DataSourceType;
  planConfig: PlanConfig;
}

export const CLIENT_CONFIGS: Record<ClientId, ClientConfig> = {
  opencode: {
    id: "opencode",
    label: "OpenCode",
    dataSource: "sqlite",
    planConfig: {
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
    },
  },
  claude: {
    id: "claude",
    label: "Claude Code",
    dataSource: "jsonl",
    planConfig: {
      targetModels: [
        "claude-opus-4",
        "claude-sonnet-4",
        "claude-haiku-4",
        "<synthetic>",
      ],
      windows: [
        { label: "5h", hours: 5, limit: 20.0 },
        { label: "Weekly", hours: 7 * 24, limit: 50.0 },
        { label: "Monthly", hours: 30 * 24, limit: 100.0 },
      ],
    },
  },
  codex: {
    id: "codex",
    label: "Codex",
    dataSource: "jsonl",
    planConfig: {
      targetModels: ["gpt-5"],
      windows: [
        { label: "5h", hours: 5, limit: 15.0 },
        { label: "Weekly", hours: 7 * 24, limit: 40.0 },
        { label: "Monthly", hours: 30 * 24, limit: 80.0 },
      ],
    },
  },
  cursor: {
    id: "cursor",
    label: "Cursor",
    dataSource: "jsonl",
    planConfig: {
      targetModels: [
        "claude-",
        "gpt-4",
        "gpt-5",
        "cursor-",
      ],
      windows: [
        { label: "5h", hours: 5, limit: 20.0 },
        { label: "Weekly", hours: 7 * 24, limit: 50.0 },
        { label: "Monthly", hours: 30 * 24, limit: 100.0 },
      ],
    },
  },
  windsurf: {
    id: "windsurf",
    label: "Windsurf",
    dataSource: "jsonl",
    planConfig: {
      targetModels: [
        "claude-",
        "gpt-4",
        "gpt-5",
      ],
      windows: [
        { label: "5h", hours: 5, limit: 20.0 },
        { label: "Weekly", hours: 7 * 24, limit: 50.0 },
        { label: "Monthly", hours: 30 * 24, limit: 100.0 },
      ],
    },
  },
  qwen: {
    id: "qwen",
    label: "Qwen",
    dataSource: "jsonl",
    planConfig: {
      targetModels: [
        "coder-model",
        "qwen-",
      ],
      windows: [
        { label: "5h", hours: 5, limit: 15.0 },
        { label: "Weekly", hours: 7 * 24, limit: 40.0 },
        { label: "Monthly", hours: 30 * 24, limit: 80.0 },
      ],
    },
  },
};

export const ALL_CLIENTS: ClientId[] = [
  "opencode",
  "claude",
  "codex",
  "cursor",
  "windsurf",
  "qwen",
];
