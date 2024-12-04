import assert from "node:assert";
import { describe, it } from "node:test";
import { getFileHandle } from "../utils";
import day1 from "../days/day1";
import day2 from "../days/day2";
import day3 from "../days/day3";
import day4 from "../days/day4";

describe("day1 function", () => {
  it("task 1 (test) should return 11", async () => {
    const dayFileHandle = await getFileHandle("1-1", true);
    const dayResult = await day1(1, dayFileHandle);
    assert.strictEqual(dayResult, "11");
  });

  it("task 1 should return 2176849", async () => {
    const dayFileHandle = await getFileHandle("1-1");
    const dayResult = await day1(1, dayFileHandle);
    assert.strictEqual(dayResult, "2176849");
  });

  it("task 2 (test) should return 31", async () => {
    const dayFileHandle = await getFileHandle("1-2", true);
    const dayResult = await day1(2, dayFileHandle);
    assert.strictEqual(dayResult, "31");
  });

  it("task 2 should return 23384288", async () => {
    const dayFileHandle = await getFileHandle("1-2");
    const dayResult = await day1(2, dayFileHandle);
    assert.strictEqual(dayResult, "23384288");
  });
});

describe("day2 function", () => {
  it("task 1 (test) should return 2", async () => {
    const dayFileHandle = await getFileHandle("2-1", true);
    const dayResult = await day2(1, dayFileHandle);
    assert.strictEqual(dayResult, "2");
  });

  it("task 1 should return 564", async () => {
    const dayFileHandle = await getFileHandle("2-1");
    const dayResult = await day2(1, dayFileHandle);
    assert.strictEqual(dayResult, "564");
  });

  it("task 2 (test) should return 4", async () => {
    const dayFileHandle = await getFileHandle("2-2", true);
    const dayResult = await day2(2, dayFileHandle);
    assert.strictEqual(dayResult, "4");
  });

  it("task 2 should return 604", async () => {
    const dayFileHandle = await getFileHandle("2-2");
    const dayResult = await day2(2, dayFileHandle);
    assert.strictEqual(dayResult, "604");
  });
});

describe("day3 function", () => {
  it("task 1 (test) should return 161", async () => {
    const dayFileHandle = await getFileHandle("3-1", true);
    const dayResult = await day3(1, dayFileHandle);
    assert.strictEqual(dayResult, "161");
  });

  it("task 1 should return 182619815", async () => {
    const dayFileHandle = await getFileHandle("3-1");
    const dayResult = await day3(1, dayFileHandle);
    assert.strictEqual(dayResult, "182619815");
  });

  it("task 2 (test) should return 48", async () => {
    const dayFileHandle = await getFileHandle("3-2", true);
    const dayResult = await day3(2, dayFileHandle);
    assert.strictEqual(dayResult, "48");
  });

  it("task 2 should return 604", async () => {
    const dayFileHandle = await getFileHandle("3-2");
    const dayResult = await day3(2, dayFileHandle);
    assert.strictEqual(dayResult, "80747545");
  });
});

describe("day4 function", () => {
  it("task 1 (test) should return 16", async () => {
    const dayFileHandle = await getFileHandle("4-1", true);
    const dayResult = await day4(1, dayFileHandle);
    assert.strictEqual(dayResult, "18");
  });

  it("task 1 should return 2464", async () => {
    const dayFileHandle = await getFileHandle("4-1");
    const dayResult = await day4(1, dayFileHandle);
    assert.strictEqual(dayResult, "2464");
  });

  it("task 2 (test) should return 9", async () => {
    const dayFileHandle = await getFileHandle("4-2", true);
    const dayResult = await day4(2, dayFileHandle);
    assert.strictEqual(dayResult, "9");
  });

  it("task 2 should return 1982", async () => {
    const dayFileHandle = await getFileHandle("4-2");
    const dayResult = await day4(2, dayFileHandle);
    assert.strictEqual(dayResult, "1982");
  });
});
