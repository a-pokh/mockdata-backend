CREATE TABLE projects (
  id TEXT PRIMARY KEY,
  title VARCHAR NOT NULL,
  "description" TEXT,
  "connection_string" TEXT,
  "ddl_schema" TEXT
)