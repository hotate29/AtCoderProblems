import React from "react";
import * as Url from "../utils/Url";
import Contest from "../interfaces/Contest";
import { getRatingColorClass } from "../utils";
import { AGC_001_START } from "../utils/ContestClassifier";
import { NewTabLink } from "./NewTabLink";

interface Props {
  contest: Contest;
  title?: string;
}

export const AllRated = Symbol();
export const Unrated = Symbol();
type RatedTargetType = typeof AllRated | typeof Unrated;

export type RatedTarget = number | RatedTargetType;

export function getRatedTarget(contest: Contest): RatedTarget {
  if (AGC_001_START > contest.start_epoch_second) {
    return Unrated;
  }
  switch (contest.rate_change) {
    case undefined:
      return Unrated;
    case "-":
      return Unrated;
    case "All":
      return AllRated;
    default: {
      const range = contest.rate_change.split("~").map((r) => r.trim());
      if (range.length !== 2) {
        return Unrated;
      }
      const upperBound = parseInt(range[1]);
      if (upperBound) {
        return upperBound;
      }
      const lowerBound = parseInt(range[0]);
      if (lowerBound) {
        return AllRated;
      }
      return Unrated;
    }
  }
}

function getColorClass(target: RatedTarget): string {
  if (target === AllRated) {
    return "difficulty-red";
  }
  if (target === Unrated) {
    return "";
  }

  return getRatingColorClass(target);
}

export const ContestLink: React.FC<Props> = (props) => {
  const { contest, title } = props;
  const target: RatedTarget = getRatedTarget(contest);

  return (
    <>
      <span className={getColorClass(target)}>◉</span>{" "}
      <NewTabLink href={Url.formatContestUrl(contest.id)}>
        {title !== undefined ? title : contest.title}
      </NewTabLink>
    </>
  );
};
