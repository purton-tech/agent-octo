pub mod agents {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/agents")]
    pub struct Index {
        pub org_id: String,
    }
}
