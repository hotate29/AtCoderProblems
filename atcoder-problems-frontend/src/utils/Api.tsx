import { UserId } from "../interfaces/Status";
import Submission, { isSubmission } from "../interfaces/Submission";
import { hasPropertyAsType, isNumber } from "./TypeUtils";

const ATCODER_API_URL = import.meta.env.VITE_ATCODER_API_URL;

export const fetchPartialUserSubmissions = async (
  userId: UserId,
  fromSecond: number
): Promise<Submission[]> => {
  if (userId.length === 0) {
    return [];
  }
  const url = `${ATCODER_API_URL}/v3/user/submissions?user=${userId}&from_second=${fromSecond}`;
  const response = await fetch(url);

  const json: unknown = await response.json();
  if (!Array.isArray(json)) {
    return [];
  }
  return json.filter(isSubmission);
};

export const fetchUserSubmissionCount = async (
  userId: string,
  fromSecond: number,
  toSecond: number
) => {
  const url = `${ATCODER_API_URL}/v3/user/submission_count?user=${userId}&from_second=${fromSecond}&to_second=${toSecond}`;
  const response = await fetch(url);
  const json: unknown = await response.json();
  if (hasPropertyAsType(json, "count", isNumber)) {
    return json.count;
  } else {
    return 0;
  }
};
