use axum::http::StatusCode;
use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::get;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

mod projects {
    use super::*;
    pub(crate) mod v2 {
        use super::*;

        #[utoipa::path(
            get,
            path = "/v2/projects",
            responses((status = StatusCode::OK, body = Vec<Uuid>)),
        )]
        pub async fn get_projects(
        ) -> impl IntoResponse {
            (StatusCode::OK, Json::<Vec<Uuid>>(Vec::from([
                Uuid::new_v4(),
                Uuid::new_v4(),
            ]))).into_response()
        }
    }

    pub(crate) mod v3 {
        use super::*;

        #[utoipa::path(
            get,
            path = "/v3/projects",
            responses((status = StatusCode::OK, body = Vec<Uuid>)),
        )]
        pub async fn get_projects(
        ) -> impl IntoResponse {
            (StatusCode::OK, Json::<Vec<Uuid>>(Vec::from([
                Uuid::new_v4(),
                Uuid::new_v4(),
                Uuid::new_v4(),
            ]))).into_response()
        }
    }
}

pub mod docs {
    use utoipa::OpenApi;
    use crate::projects::v2;
    use crate::projects::v3;

    #[derive(OpenApi)]
    #[openapi(
        paths(
            v3::get_projects,
            v2::get_projects,
        )
    )]
    pub(crate) struct ApiDoc;
}

#[tokio::main]
async fn main() {
    use projects::v2;
    use projects::v3;
    let test_app = Router::new()
        .route("/v3/projects", get(v3::get_projects))
        .route("/v2/projects", get(v2::get_projects))
        .merge(SwaggerUi::new("/docs").url("/openapi.json", docs::ApiDoc::openapi()));

    let addr = std::format!("0.0.0.0:{}", 2222);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|error| { panic!("{:?}", error) });

    axum::serve(listener, test_app).await.expect("panic!!!");
}
