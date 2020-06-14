use serde_derive::{Deserialize, Serialize};
use diesel::{Insertable, Queryable};
use crate::schema::{projects, project_tables, project_table_fields};

#[derive(Identifiable, Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub connection_string: Option<String>,
    pub ddl_schema: Option<String>,
    pub database_type: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name="projects"]
pub struct NewProject {
    pub title: String,
    pub description: Option<String>,
    pub connection_string: Option<String>,
    pub ddl_schema: Option<String>,
    pub database_type: String,
}

#[derive(Identifiable,Insertable, Queryable, AsChangeset, Serialize, Deserialize, Associations)]
#[belongs_to(Project)]
pub struct ProjectTable {
    pub id: String,
    pub name: String,
    pub schema: String,
    pub project_id: String,
}

#[derive(Identifiable, Insertable, Queryable, AsChangeset, Serialize, Deserialize, Associations)]
#[belongs_to(ProjectTable)]
pub struct ProjectTableField {
    pub id: String,
    pub name: String,
    pub data_type: String,
    pub reference_table: Option<String>,
    pub fake_data_type: Option<String>,
    pub project_table_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectTableFieldView {
    pub id: String,
    pub name: String,
    pub data_type: String,
    pub fake_data_type: Option<String>,
    pub reference_table: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectTableView {
    pub id: String,
    pub name: String,
    pub schema: String,
    pub fields: Vec<ProjectTableFieldView>
}