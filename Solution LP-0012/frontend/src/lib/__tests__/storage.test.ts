import { describe, expect, it } from "vitest";
import { createJsonStore } from "../storage";

class MockStorage {
  private map = new Map<string, string>();
  getItem(key: string) {
    return this.map.get(key) ?? null;
  }
  setItem(key: string, value: string) {
    this.map.set(key, value);
  }
  removeItem(key: string) {
    this.map.delete(key);
  }
  keys() {
    return [...this.map.keys()];
  }
}

describe("json store", () => {
  it("persists and loads values", () => {
    const store = createJsonStore(new MockStorage() as never, "test");
    store.set("one", { a: 1 });
    expect(store.get("one", { a: 0 })).toEqual({ a: 1 });
  });
});
