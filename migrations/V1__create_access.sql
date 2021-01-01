CREATE TABLE "access" (
  "name" varchar NOT NULL,
  "single_use" integer NOT NULL DEFAULT 1,
  "admin" integer NOT NULL DEFAULT 0,
  "session_id" varchar NULL,
  "token" varchar NOT NULL,
  "whitelist" text NULL
);
