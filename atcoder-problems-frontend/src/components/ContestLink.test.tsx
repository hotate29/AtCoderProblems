import { AGC_001_START } from "../utils/ContestClassifier";
import { AllRated, Unrated, getRatedTarget } from "./ContestLink";

const DEFAULT_CONTEST = {
  id: "",
  contest_id: "",
  title: "",
  start_epoch_second: AGC_001_START,
  duration_second: 0,
  rate_change: "-",
};

describe("Infer rating change of contests", () => {
  it("ARC level", () => {
    const contest = {
      ...DEFAULT_CONTEST,
      rate_change: " ~ 2799",
    };

    expect(getRatedTarget(contest)).toBe(2799);
  });
  it("new ABC level", () => {
    const contest = {
      ...DEFAULT_CONTEST,
      rate_change: " ~ 1999",
    };

    expect(getRatedTarget(contest)).toBe(1999);
  });
  it("old ABC level", () => {
    const contest = {
      ...DEFAULT_CONTEST,
      rate_change: " ~ 1119",
    };

    expect(getRatedTarget(contest)).toBe(1119);
  });
  it("old AGC level", () => {
    const contest = {
      ...DEFAULT_CONTEST,
      rate_change: "All",
    };

    expect(getRatedTarget(contest)).toBe(AllRated);
  });
  it("new AGC level", () => {
    const contest = {
      ...DEFAULT_CONTEST,
      rate_change: "1200 ~",
    };

    expect(getRatedTarget(contest)).toBe(AllRated);
  });
  it("buggy unrated", () => {
    const contest = {
      ...DEFAULT_CONTEST,
      start_epoch_second: 0,
      rate_change: "All",
    };

    expect(getRatedTarget(contest)).toBe(Unrated);
  });
  it("unrated", () => {
    const contest = {
      ...DEFAULT_CONTEST,
      rate_change: "-",
    };

    expect(getRatedTarget(contest)).toBe(Unrated);
  });
});
