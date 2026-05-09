import { describe, it, expect } from "vitest";
import {
  filterTargetMessages,
  calculateWindowCost,
  calculateWindowMetrics,
  calculateNextFreeSlot,
  aggregateAllModels,
} from "./calculator";
import type { UnifiedMessage } from "./types";
import { GO_PLAN_CONFIG } from "./planConfig";

const testPlanConfig = GO_PLAN_CONFIG;

describe("filterTargetMessages", () => {
  it("includes only target models", () => {
    const messages: UnifiedMessage[] = [
      { id: "1", client: "opencode", model_id: "kimi-k2.6", cost: 1.0, timestamp: 0 },
      { id: "2", client: "opencode", model_id: "unknown-model", cost: 1.0, timestamp: 0 },
      { id: "3", client: "opencode", model_id: "Kimi-K2.5", cost: 1.0, timestamp: 0 },
    ];
    const result = filterTargetMessages(messages, testPlanConfig);
    expect(result).toHaveLength(2);
    expect(result.map((m) => m.model_id)).toEqual(["kimi-k2.6", "Kimi-K2.5"]);
  });

  it("returns empty array when no messages match", () => {
    const messages: UnifiedMessage[] = [
      { id: "1", client: "opencode", model_id: "other", cost: 1.0, timestamp: 0 },
    ];
    expect(filterTargetMessages(messages, testPlanConfig)).toHaveLength(0);
  });
});

describe("calculateWindowCost", () => {
  const now = new Date("2024-01-15T12:00:00Z").getTime();
  const fiveHoursMs = 5 * 60 * 60 * 1000;

  it("includes message exactly at the boundary", () => {
    const messages: UnifiedMessage[] = [
      { id: "1", client: "opencode", model_id: "kimi-k2.6", cost: 2.5, timestamp: now - fiveHoursMs },
    ];
    expect(calculateWindowCost(messages, now, 5)).toBe(2.5);
  });

  it("excludes message just before the boundary", () => {
    const messages: UnifiedMessage[] = [
      { id: "1", client: "opencode", model_id: "kimi-k2.6", cost: 2.5, timestamp: now - fiveHoursMs - 1 },
    ];
    expect(calculateWindowCost(messages, now, 5)).toBe(0);
  });

  it("excludes message in the future", () => {
    const messages: UnifiedMessage[] = [
      { id: "1", client: "opencode", model_id: "kimi-k2.6", cost: 2.5, timestamp: now + 1000 },
    ];
    expect(calculateWindowCost(messages, now, 5)).toBe(0);
  });

  it("sums multiple messages within the window", () => {
    const messages: UnifiedMessage[] = [
      { id: "1", client: "opencode", model_id: "kimi-k2.6", cost: 1.0, timestamp: now - 1 * 60 * 60 * 1000 },
      { id: "2", client: "opencode", model_id: "kimi-k2.6", cost: 2.0, timestamp: now - 2 * 60 * 60 * 1000 },
      { id: "3", client: "opencode", model_id: "kimi-k2.6", cost: 3.0, timestamp: now - 6 * 60 * 60 * 1000 },
    ];
    expect(calculateWindowCost(messages, now, 5)).toBe(3.0);
  });
});

describe("calculateWindowMetrics", () => {
  it("calculates remaining and usage rate correctly", () => {
    const metrics = calculateWindowMetrics(6.0, 12.0);
    expect(metrics.cost).toBe(6.0);
    expect(metrics.remaining).toBe(6.0);
    expect(metrics.usageRate).toBe(0.5);
    expect(metrics.isWarning).toBe(false);
    expect(metrics.limit).toBe(12.0);
  });

  it("triggers warning when usage exceeds 80%", () => {
    const metrics = calculateWindowMetrics(10.0, 12.0);
    expect(metrics.usageRate).toBeCloseTo(0.8333, 3);
    expect(metrics.isWarning).toBe(true);
  });

  it("does not trigger warning at exactly 80%", () => {
    const metrics = calculateWindowMetrics(9.6, 12.0);
    expect(metrics.usageRate).toBeCloseTo(0.8, 10);
    expect(metrics.isWarning).toBe(false);
  });

  it("caps remaining at zero when over limit", () => {
    const metrics = calculateWindowMetrics(15.0, 12.0);
    expect(metrics.remaining).toBe(0);
  });
});

describe("calculateNextFreeSlot", () => {
  const now = new Date("2024-01-15T12:00:00Z").getTime();
  const fiveHoursMs = 5 * 60 * 60 * 1000;

  it("returns null for empty window", () => {
    const result = calculateNextFreeSlot([], now, 5);
    expect(result.freeAt).toBeNull();
    expect(result.minutesUntil).toBeNull();
  });

  it("calculates next free slot from oldest message", () => {
    const messages: UnifiedMessage[] = [
      { id: "1", client: "opencode", model_id: "kimi-k2.6", cost: 1.0, timestamp: now - 3 * 60 * 60 * 1000 },
      { id: "2", client: "opencode", model_id: "kimi-k2.6", cost: 1.0, timestamp: now - 1 * 60 * 60 * 1000 },
    ];
    const result = calculateNextFreeSlot(messages, now, 5);
    const expectedFreeAt = now - 3 * 60 * 60 * 1000 + fiveHoursMs;
    expect(result.freeAt).toBe(expectedFreeAt);
    expect(result.minutesUntil).toBe(120);
  });
});

describe("aggregateAllModels", () => {
  const now = new Date("2024-01-15T12:00:00Z").getTime();

  it("aggregates models across clients", () => {
    const clientMessages = {
      opencode: [
        { id: "1", client: "opencode", model_id: "Kimi-K2.6", cost: 5.0, timestamp: now - 1 * 60 * 60 * 1000 },
        { id: "2", client: "opencode", model_id: "Kimi-K2.5", cost: 3.0, timestamp: now - 2 * 60 * 60 * 1000 },
      ] as UnifiedMessage[],
      claude: [
        { id: "3", client: "claude", model_id: "claude-4-sonnet", cost: 8.0, timestamp: now - 1 * 60 * 60 * 1000 },
      ] as UnifiedMessage[],
    };

    const clientLabels = {
      opencode: "OpenCode",
      claude: "Claude Code",
    };

    const planConfigs = {
      opencode: GO_PLAN_CONFIG,
      claude: {
        targetModels: ["claude-4-sonnet", "claude-4-opus"],
        windows: [{ label: "5h", hours: 5, limit: 20.0 }],
      },
    };

    const result = aggregateAllModels(clientMessages, clientLabels, planConfigs, now, 5);

    expect(result).toHaveLength(3);
    const kimiK26 = result.find((r) => r.modelId === "Kimi-K2.6");
    expect(kimiK26?.cost).toBe(5.0);
    expect(kimiK26?.client).toBe("opencode");
    expect(kimiK26?.usageRate).toBe(5.0 / 12.0);

    const claudeSonnet = result.find((r) => r.modelId === "claude-4-sonnet");
    expect(claudeSonnet?.cost).toBe(8.0);
    expect(claudeSonnet?.client).toBe("claude");
    expect(claudeSonnet?.usageRate).toBe(8.0 / 20.0);
  });

  it("returns empty array when no messages", () => {
    const result = aggregateAllModels({}, {}, {}, now, 5);
    expect(result).toHaveLength(0);
  });
});
