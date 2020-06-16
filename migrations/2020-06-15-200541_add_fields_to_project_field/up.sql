ALTER TABLE public."project_table_fields" ADD "is_not_null" BOOLEAN NOT NULL DEFAULT(false);
ALTER TABLE public."project_table_fields" ADD "is_primary_key" BOOLEAN NOT NULL DEFAULT(false);
ALTER TABLE public."project_table_fields" ADD "is_unique" BOOLEAN NOT NULL DEFAULT(false);