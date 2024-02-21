/// <reference types="react-scripts" />
declare namespace NodeJS {
  interface ProcessEnv {
    NODE_ENV: "development" | "production" | "test";
    PUBLIC_URL: string;
    VITE_INTERNAL_API_URL: string;
    VITE_ATCODER_API_URL: string;
  }
}
