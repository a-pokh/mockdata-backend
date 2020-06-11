use warp::Filter;
use crate::handlers::project::*;

pub fn project_filters() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    projects_list()
        .or(project_get_one())
        .or(project_get_tables())
        .or(projects_introspect())
        .or(projects_create())
        .or(projects_update())
        .or(projects_delete())
}

pub fn projects_list() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("projects")
        .and(warp::get())
        .and_then(list_projects)
}

pub fn project_get_one() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("projects" / String)
        .and(warp::get())
        .and_then(get_project)
}

pub fn project_get_tables() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("projects" / String / "tables")
        .and(warp::get())
        .and_then(get_project_tables)
}

pub fn projects_introspect() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("projects" / String / "introspect")
        .and(warp::post())
        .and_then(introspect_project)
}

pub fn projects_create() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("projects")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(create_project)
}

pub fn projects_update() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("projects" / String)
        .and(warp::put())
        .and(warp::body::json())
        .and_then(update_project)
}

pub fn projects_delete() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("projects" / String)
        .and(warp::delete())
        .and_then(delete_project)
}
