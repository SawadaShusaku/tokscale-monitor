import { describe, it, expect } from "vitest";

describe("ProgressBar warning logic", () => {
  function getIsWarning(usageRate: number): boolean {
    return usageRate > 0.8;
  }

  it("does not warn at 80% usage", () => {
    expect(getIsWarning(0.8)).toBe(false);
  });

  it("warns above 80% usage", () => {
    expect(getIsWarning(0.81)).toBe(true);
    expect(getIsWarning(1.0)).toBe(true);
  });

  it("does not warn below 80% usage", () => {
    expect(getIsWarning(0.0)).toBe(false);
    expect(getIsWarning(0.5)).toBe(false);
    expect(getIsWarning(0.79)).toBe(false);
  });
});
