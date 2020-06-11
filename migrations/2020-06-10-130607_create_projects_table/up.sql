CREATE TABLE project_tables (
  id TEXT PRIMARY KEY,
  "name" VARCHAR NOT NULL,
  "schema" VARCHAR NOT NULL,
  "project_id" TEXT NOT NULL,
  constraint fk_project_project_tables
     foreign key (project_id) 
     REFERENCES projects (id)
     ON DELETE CASCADE
)