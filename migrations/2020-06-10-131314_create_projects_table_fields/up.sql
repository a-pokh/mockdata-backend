CREATE TABLE project_table_fields (
  id TEXT PRIMARY KEY,
  "name" TEXT NOT NULL,
  "data_type" TEXT NOT NULL,
  "reference_table" TEXT,
  "fake_data_type" TEXT,
  "project_table_id" TEXT NOT NULL,
  constraint fk_project_project_tables_fileds
     foreign key (project_table_id) 
     REFERENCES project_tables (id)
     ON DELETE CASCADE
)