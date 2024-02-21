/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_INTERNAL_API_URL: string;
  readonly VITE_ATCODER_API_URL: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
