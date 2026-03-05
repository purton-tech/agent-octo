pub mod agents {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/agents")]
    pub struct Index {
        pub org_id: String,
    }
}

pub mod channels {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/channels")]
    pub struct Index {
        pub org_id: String,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/channels/connect-telegram")]
    pub struct ConnectTelegram {
        pub org_id: String,
    }
}

pub mod providers {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/providers")]
    pub struct Index {
        pub org_id: String,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/providers/new")]
    pub struct New {
        pub org_id: String,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/providers/create")]
    pub struct Create {
        pub org_id: String,
    }
}
