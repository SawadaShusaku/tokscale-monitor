import type { UnifiedMessage } from "./types";
import type { PlanConfig } from "./planConfig";

export function filterTargetMessages(
  messages: UnifiedMessage[],
  planConfig: PlanConfig
): UnifiedMessage[] {
  const prefixes = planConfig.targetModels.map((m) => m.toLowerCase());
  return messages.filter((m) => {
    const model = m.model_id.toLowerCase();
    return prefixes.some((prefix) => model.startsWith(prefix));
  });
}

export function calculateWindowCost(
  messages: UnifiedMessage[],
  now: number,
  hours: number
): number {
  const cutoff = now - hours * 60 * 60 * 1000;
  return messages
    .filter((m) => {
      const t = m.timestamp;
      return cutoff <= t && t < now;
    })
    .reduce((sum, m) => sum + m.cost, 0);
}

export interface WindowMetrics {
  cost: number;
  remaining: number;
  usageRate: number;
  isWarning: boolean;
  limit: number;
}

export function calculateWindowMetrics(
  cost: number,
  limit: number
): WindowMetrics {
  const usageRate = limit > 0 ? cost / limit : 0;
  return {
    cost,
    remaining: Math.max(0, limit - cost),
    usageRate,
    isWarning: usageRate > 0.8,
    limit,
  };
}

export function calculateNextFreeSlot(
  messages: UnifiedMessage[],
  now: number,
  hours: number
): { freeAt: number | null; minutesUntil: number | null } {
  const cutoff = now - hours * 60 * 60 * 1000;
  const windowMessages = messages.filter((m) => {
    const t = m.timestamp;
    return cutoff <= t && t < now;
  });

  if (windowMessages.length === 0) {
    return { freeAt: null, minutesUntil: null };
  }

  const oldest = windowMessages.reduce((min, m) => {
    const t = m.timestamp;
    return t < min ? t : min;
  }, Infinity);

  const freeAt = oldest + hours * 60 * 60 * 1000;
  const minutesUntil = Math.max(0, Math.ceil((freeAt - now) / (60 * 1000)));
  return { freeAt, minutesUntil };
}

export interface ModelAggregate {
  modelId: string;
  client: string;
  clientLabel: string;
  cost: number;
  limit: number;
  usageRate: number;
  isWarning: boolean;
}

export type OverviewMode = "provider" | "model";

export function aggregateAllModels(
  clientMessages: Record<string, UnifiedMessage[]>,
  clientLabels: Record<string, string>,
  planConfigs: Record<string, PlanConfig>,
  now: number,
  windowHours: number
): ModelAggregate[] {
  const results: ModelAggregate[] = [];

  for (const [clientId, messages] of Object.entries(clientMessages)) {
    const planConfig = planConfigs[clientId];
    const label = clientLabels[clientId] || clientId;
    if (!planConfig) continue;

    const targetMessages = filterTargetMessages(messages, planConfig);
    const cutoff = now - windowHours * 60 * 60 * 1000;
    const windowMessages = targetMessages.filter((m) => {
      const t = m.timestamp;
      return cutoff <= t && t < now;
    });

    // Group by model
    const modelCosts = new Map<string, number>();
    for (const m of windowMessages) {
      const current = modelCosts.get(m.model_id) || 0;
      modelCosts.set(m.model_id, current + m.cost);
    }

    // Find matching window config by hours, fallback to first window
    const windowConfig = planConfig.windows.find((w) => w.hours === windowHours);
    const windowLimit = windowConfig?.limit || planConfig.windows[0]?.limit || 0;

    for (const [modelId, cost] of modelCosts.entries()) {
      const metrics = calculateWindowMetrics(cost, windowLimit);
      results.push({
        modelId,
        client: clientId,
        clientLabel: label,
        cost,
        limit: windowLimit,
        usageRate: metrics.usageRate,
        isWarning: metrics.isWarning,
      });
    }
  }

  // Sort by usage rate descending
  return results.sort((a, b) => b.usageRate - a.usageRate);
}

export function aggregateAllProviders(
  clientMessages: Record<string, UnifiedMessage[]>,
  clientLabels: Record<string, string>,
  planConfigs: Record<string, PlanConfig>,
  now: number,
  windowHours: number
): ModelAggregate[] {
  const results: ModelAggregate[] = [];

  for (const [clientId, messages] of Object.entries(clientMessages)) {
    const planConfig = planConfigs[clientId];
    const label = clientLabels[clientId] || clientId;
    if (!planConfig) continue;

    const targetMessages = filterTargetMessages(messages, planConfig);
    const cutoff = now - windowHours * 60 * 60 * 1000;
    const windowMessages = targetMessages.filter((m) => {
      const t = m.timestamp;
      return cutoff <= t && t < now;
    });

    const totalCost = windowMessages.reduce((sum, m) => sum + m.cost, 0);

    // Find matching window config by hours, fallback to first window
    const windowConfig = planConfig.windows.find((w) => w.hours === windowHours);
    const windowLimit = windowConfig?.limit || planConfig.windows[0]?.limit || 0;

    const metrics = calculateWindowMetrics(totalCost, windowLimit);
    results.push({
      modelId: label,
      client: clientId,
      clientLabel: label,
      cost: totalCost,
      limit: windowLimit,
      usageRate: metrics.usageRate,
      isWarning: metrics.isWarning,
    });
  }

  // Sort by usage rate descending
  return results.sort((a, b) => b.usageRate - a.usageRate);
}
