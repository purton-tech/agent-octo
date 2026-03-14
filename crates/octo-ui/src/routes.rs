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

pub mod billing {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/billing")]
    pub struct Index {
        pub org_id: String,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/billing/start-checkout")]
    pub struct StartCheckout {
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

pub mod integrations {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/integrations")]
    pub struct Index {
        pub org_id: String,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/integrations/new")]
    pub struct New {
        pub org_id: String,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/integrations/edit/{id}")]
    pub struct Edit {
        pub org_id: String,
        pub id: String,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/integrations/upsert")]
    pub struct Upsert {
        pub org_id: String,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/integrations/delete/{id}")]
    pub struct Delete {
        pub org_id: String,
        pub id: String,
    }
}

pub mod connections {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/connections")]
    pub struct Index {
        pub org_id: String,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/connections/new")]
    pub struct New {
        pub org_id: String,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/connections/create")]
    pub struct Create {
        pub org_id: String,
    }
}
