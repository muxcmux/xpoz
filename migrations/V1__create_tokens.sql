CREATE TABLE "tokens" (
  "name" varchar NOT NULL,
  "session_bound" integer NOT NULL DEFAULT 1,
  "admin" integer NOT NULL DEFAULT 0,
  "session_id" varchar NULL,
  "token" varchar NOT NULL,
  "whitelist" text NULL,
  "created_at" datetime NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX "token" ON "tokens" ("token");
CREATE INDEX "session_id" ON "tokens" ("session_id");
