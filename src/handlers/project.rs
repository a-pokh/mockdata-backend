use crate::models::{
    NewProject, Project, ProjectTable, ProjectTableField, ProjectTableFieldView, ProjectTableView,
};
use crate::schema::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use fakedata;
use mockdata_ddl;
use std::convert::Infallible;
use std::env;
use std::thread;
use uuid::Uuid;
use warp::http::StatusCode;

/*
TODO:
error handling,
move connection creation outside handlers,
consider repository pattern,
consider r2d2 pools,
another comment
...
*/

pub async fn list_projects() -> Result<impl warp::Reply, Infallible> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url).unwrap();

    let query = projects::table.load::<Project>(&conn).unwrap();

    Ok(warp::reply::json(&query))
}

pub async fn get_project(project_id: String) -> Result<impl warp::Reply, Infallible> {
    use crate::schema::projects::dsl::projects;
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url).unwrap();

    let project = projects
        .find(project_id)
        .get_result::<Project>(&conn)
        .expect("Error saving new project");

    Ok(warp::reply::json(&project))
}

pub async fn get_project_tables(project_id: String) -> Result<impl warp::Reply, Infallible> {
    use crate::schema::projects::dsl::projects;
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url).unwrap();

    let project = projects
        .find(project_id)
        .get_result::<Project>(&conn)
        .expect("Error saving new project");
    let project_tables = ProjectTable::belonging_to(&project)
        .load::<ProjectTable>(&conn)
        .expect("Error saving new project");
    let project_table_fields = ProjectTableField::belonging_to(&project_tables)
        .load::<ProjectTableField>(&conn)
        .expect("Error saving new project");

    let grouped_fields: Vec<Vec<ProjectTableField>> =
        project_table_fields.grouped_by(&project_tables);
    let tables_and_fields: Vec<(ProjectTable, Vec<ProjectTableField>)> =
        project_tables.into_iter().zip(grouped_fields).collect();

    let mut result = Vec::new();
    for table_and_fields in tables_and_fields {
        let (project_table, fields) = table_and_fields;

        let fields_views: Vec<ProjectTableFieldView> = fields
            .into_iter()
            .map(|f| ProjectTableFieldView {
                id: f.id.clone(),
                name: f.name.clone(),
                data_type: f.data_type.clone(),
                fake_data_type: f.fake_data_type.clone(),
                reference_table: f.reference_table.clone(),
                is_not_null: f.is_not_null.clone(),
                is_primary_key: f.is_primary_key.clone(),
                is_unique: f.is_unique.clone(),
                enum_values: f.enum_values.clone(),
            })
            .collect();

        let table_view = ProjectTableView {
            id: project_table.id.clone(),
            name: project_table.name.clone(),
            schema: project_table.schema.clone(),
            fields: fields_views,
            generate_data_count: project_table.generate_data_count,
        };

        result.push(table_view);
    }

    Ok(warp::reply::json(&result))
}

pub async fn introspect_project(project_id: String) -> Result<impl warp::Reply, Infallible> {
    use crate::schema::projects::dsl::projects;
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url).unwrap();

    diesel::delete(project_tables::table)
        .execute(&conn)
        .expect("Error saving new project");

    let project = projects
        .find(&project_id)
        .get_result::<Project>(&conn)
        .expect("Error saving new project");

    let database_type = project.database_type;
    let schema_name = project.database_schema.unwrap();
    let connection_string = project.connection_string.unwrap();

    let processing_thread = thread::spawn(move || {
        mockdata_ddl::get_database_structure(&database_type, &connection_string, &schema_name)
    });

    let database_parsed_ddl = processing_thread.join().unwrap();

    let mut project_tables = Vec::new();
    let mut project_table_fields = Vec::new();
    for table in &database_parsed_ddl.unwrap() {
        let table_id = Uuid::new_v4().to_string();

        project_tables.push(ProjectTable {
            id: table_id.clone(),
            project_id: project_id.clone(),
            name: table.name.clone(),
            schema: table.schema.clone(),
            // this is default value in DB, not sure how to skip its insertion
            generate_data_count: 10,
        });

        for field in &table.fields {
            let enum_values = field.enum_values.as_ref().map(|e| e.join(","));
            let fake_data_type = fakedata::get_data_type(
                &field.name,
                &field.data_type,
                *&field.is_primary_key,
                *&table.has_composite_primary_key(),
                *&field.reference_table.is_some(),
                enum_values.is_some(),
                *&field.is_unique,
            );

            project_table_fields.push(ProjectTableField {
                id: Uuid::new_v4().to_string(),
                project_table_id: table_id.clone(),
                name: field.name.clone(),
                data_type: field.data_type.clone(),
                reference_table: field.reference_table.clone(),
                is_not_null: field.is_not_null,
                is_primary_key: field.is_primary_key,
                is_unique: field.is_unique,
                fake_data_type,
                enum_values,
            });
        }
    }

    diesel::insert_into(project_tables::table)
        .values(&project_tables)
        .execute(&conn)
        .expect("Error saving new project");
    diesel::insert_into(project_table_fields::table)
        .values(&project_table_fields)
        .execute(&conn)
        .expect("Error saving new project");

    Ok(StatusCode::OK)
}

pub async fn generate_project_data(project_id: String) -> Result<impl warp::Reply, Infallible> {
    use crate::schema::projects::dsl::projects;
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url).unwrap();

    let project = projects
        .find(project_id)
        .get_result::<Project>(&conn)
        .expect("Error saving new project");
    let project_tables = ProjectTable::belonging_to(&project)
        .load::<ProjectTable>(&conn)
        .expect("Error saving new project");
    let project_table_fields = ProjectTableField::belonging_to(&project_tables)
        .load::<ProjectTableField>(&conn)
        .expect("Error saving new project");

    let grouped_fields: Vec<Vec<ProjectTableField>> =
        project_table_fields.grouped_by(&project_tables);
    let tables_and_fields: Vec<(ProjectTable, Vec<ProjectTableField>)> =
        project_tables.into_iter().zip(grouped_fields).collect();

    let mut result = Vec::new();
    for table_and_fields in tables_and_fields {
        let (project_table, fields) = table_and_fields;

        let fields: Vec<fakedata::data_generator::GeneratorField> = fields
            .into_iter()
            .map(|f| fakedata::data_generator::GeneratorField {
                name: f.name.clone(),
                data_type: f.fake_data_type.clone(),
                reference_table: f.reference_table.clone(),
                is_not_null: f.is_not_null.clone(),
                is_primary_key: f.is_primary_key.clone(),
                is_unique: f.is_unique.clone(),
                enum_values: match f.enum_values {
                    Some(values) => Some(values.split(",").map(|s| String::from(s)).collect()),
                    None => None,
                },
            })
            .collect();

        let table = fakedata::data_generator::GeneratorTable {
            name: project_table.name.clone(),
            schema: project_table.schema.clone(),
            fields,
            count: project_table.generate_data_count as u32,
        };

        result.push(table);
    }

    let generated = fakedata::generate_data(result);

    Ok(warp::reply::json(&generated))
}

pub async fn create_project(create: NewProject) -> Result<impl warp::Reply, Infallible> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url).unwrap();

    let project = Project {
        title: create.title,
        description: create.description,
        database_type: create.database_type,
        connection_string: create.connection_string,
        ddl_schema: create.ddl_schema,
        id: Uuid::new_v4().to_string(),
        database_schema: create.database_schema,
    };

    let project: Project = diesel::insert_into(projects::table)
        .values(&project)
        .get_result(&conn)
        .expect("Error saving new project");

    Ok(warp::reply::json(&project))
}

pub async fn update_project(
    id: String,
    update: NewProject,
) -> Result<impl warp::Reply, Infallible> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url).unwrap();

    let project = Project {
        id: String::from(&id),
        title: update.title,
        database_type: update.database_type,
        description: update.description,
        connection_string: update.connection_string,
        ddl_schema: update.ddl_schema,
        database_schema: update.database_schema,
    };

    let project: Project = diesel::update(projects::table.find(&id))
        .set(&project)
        .get_result(&conn)
        .expect("Error saving new project");

    Ok(warp::reply::json(&project))
}

pub async fn delete_project(id: String) -> Result<impl warp::Reply, Infallible> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url).unwrap();

    diesel::delete(projects::table.find(&id))
        .execute(&conn)
        .expect("Error saving new project");

    Ok(StatusCode::OK)
}
