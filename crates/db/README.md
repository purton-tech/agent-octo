## The Database

We use 2 main tools to manage the database

- `dbmate` For schema migrations
- `cornucopia` for generating rust code from `sql` files.
- `just -f crates/db/Justfile db-diagram` add schema diagrams to REDME.md

## Database Schemas

Run `just -f crates/db/Justfile db-diagram` to refresh the diagrams.

<!-- schemas-start -->
### `iam`

Identity, access, roles, teams, and memberships.

```mermaid
erDiagram
```

### `integrations`

External integrations, connections, and OpenAPI specs.

```mermaid
erDiagram
```

### `llm`

Chat conversations, messages, and runtime limits.

```mermaid
erDiagram
```

### `assistants`

Prompts, categories, and project metadata for assistants.

```mermaid
erDiagram
```

### `automation`

Automation triggers and execution history.

```mermaid
erDiagram
```

### `rag`

Datasets, documents, chunks, and retrieval metadata.

```mermaid
erDiagram
```

### `model_registry`

Model providers, models, and capabilities.

```mermaid
erDiagram
```

### `storage`

Stored binary objects and references.

```mermaid
erDiagram
    buckets {
        ARRAY allowed_mime_types 
        boolean avif_autodetection 
        timestamp_with_time_zone created_at 
        bigint file_size_limit 
        text id PK 
        text name 
        uuid owner 
        text owner_id 
        boolean public 
        buckettype type 
        timestamp_with_time_zone updated_at 
    }

    buckets_analytics {
        timestamp_with_time_zone created_at 
        timestamp_with_time_zone deleted_at 
        text format 
        uuid id PK 
        text name 
        buckettype type 
        timestamp_with_time_zone updated_at 
    }

    buckets_vectors {
        timestamp_with_time_zone created_at 
        text id PK 
        buckettype type 
        timestamp_with_time_zone updated_at 
    }

    iceberg_namespaces {
        text bucket_name 
        uuid catalog_id FK 
        timestamp_with_time_zone created_at 
        uuid id PK 
        jsonb metadata 
        text name 
        timestamp_with_time_zone updated_at 
    }

    iceberg_tables {
        text bucket_name 
        uuid catalog_id FK 
        timestamp_with_time_zone created_at 
        uuid id PK 
        text location 
        text name 
        uuid namespace_id FK 
        text remote_table_id 
        text shard_id 
        text shard_key 
        timestamp_with_time_zone updated_at 
    }

    migrations {
        timestamp_without_time_zone executed_at 
        character_varying hash 
        integer id PK 
        character_varying name UK 
    }

    objects {
        text bucket_id FK 
        timestamp_with_time_zone created_at 
        uuid id PK 
        timestamp_with_time_zone last_accessed_at 
        integer level 
        jsonb metadata 
        text name 
        uuid owner 
        text owner_id 
        ARRAY path_tokens 
        timestamp_with_time_zone updated_at 
        jsonb user_metadata 
        text version 
    }

    prefixes {
        text bucket_id PK,FK 
        timestamp_with_time_zone created_at 
        integer level PK 
        text name PK 
        timestamp_with_time_zone updated_at 
    }

    s3_multipart_uploads {
        text bucket_id FK 
        timestamp_with_time_zone created_at 
        text id PK 
        bigint in_progress_size 
        text key 
        text owner_id 
        text upload_signature 
        jsonb user_metadata 
        text version 
    }

    s3_multipart_uploads_parts {
        text bucket_id FK 
        timestamp_with_time_zone created_at 
        text etag 
        uuid id PK 
        text key 
        text owner_id 
        integer part_number 
        bigint size 
        text upload_id FK 
        text version 
    }

    vector_indexes {
        text bucket_id FK 
        timestamp_with_time_zone created_at 
        text data_type 
        integer dimension 
        text distance_metric 
        text id PK 
        jsonb metadata_configuration 
        text name 
        timestamp_with_time_zone updated_at 
    }

    objects }o--|| buckets : "bucket_id"
    prefixes }o--|| buckets : "bucket_id"
    s3_multipart_uploads }o--|| buckets : "bucket_id"
    s3_multipart_uploads_parts }o--|| buckets : "bucket_id"
    iceberg_namespaces }o--|| buckets_analytics : "catalog_id"
    iceberg_tables }o--|| buckets_analytics : "catalog_id"
    vector_indexes }o--|| buckets_vectors : "bucket_id"
    iceberg_tables }o--|| iceberg_namespaces : "namespace_id"
    s3_multipart_uploads_parts }o--|| s3_multipart_uploads : "upload_id"
```

### `ops`

Operational data like audit trails and translations.

```mermaid
erDiagram
```

### `public`

Legacy schema for extensions, helpers, and compatibility objects.

```mermaid
erDiagram
    agents {
        timestamp_with_time_zone created_at 
        uuid created_by_user_id FK 
        uuid default_connection_id FK 
        text default_model 
        text description 
        uuid id PK 
        text name 
        uuid org_id FK 
        text system_prompt 
        timestamp_with_time_zone updated_at 
        resource_visibility visibility 
    }

    channels {
        text bot_token_secret_ref 
        timestamp_with_time_zone created_at 
        uuid created_by_user_id FK 
        uuid id PK 
        channel_type kind 
        text name 
        uuid org_id FK 
        timestamp_with_time_zone updated_at 
        resource_visibility visibility 
    }

    conversations {
        uuid agent_id FK 
        timestamp_with_time_zone created_at 
        uuid created_by_user_id FK 
        uuid id PK 
        uuid org_id FK 
        text title 
        timestamp_with_time_zone updated_at 
    }

    integration_connections {
        text api_key_secret_ref 
        integration_auth_type auth_type 
        timestamp_with_time_zone created_at 
        uuid created_by_user_id FK 
        uuid id PK 
        uuid integration_id FK 
        text name 
        text oauth_access_token_secret_ref 
        timestamp_with_time_zone oauth_expires_at 
        text oauth_refresh_token_secret_ref 
        uuid org_id FK 
        timestamp_with_time_zone updated_at 
        resource_visibility visibility 
    }

    integrations {
        timestamp_with_time_zone created_at 
        uuid created_by_user_id FK 
        text description 
        uuid id PK 
        text name 
        jsonb openapi_spec 
        uuid org_id FK 
        timestamp_with_time_zone updated_at 
        resource_visibility visibility 
    }

    messages {
        text content 
        uuid conversation_id FK 
        timestamp_with_time_zone created_at 
        uuid id PK 
        jsonb metadata_json 
        message_role role 
    }

    provider_connections {
        text api_key_secret_ref 
        text base_url 
        timestamp_with_time_zone created_at 
        uuid created_by_user_id FK 
        text display_name 
        uuid id PK 
        uuid org_id FK 
        text provider_kind 
        timestamp_with_time_zone updated_at 
    }

    provider_models {
        uuid connection_id FK,UK 
        timestamp_with_time_zone created_at 
        uuid id PK 
        boolean is_enabled 
        text model UK 
    }

    schema_migrations {
        character_varying version PK 
    }

    agents }o--|| provider_connections : "default_connection_id"
    conversations }o--|| agents : "agent_id"
    messages }o--|| conversations : "conversation_id"
    integration_connections }o--|| integrations : "integration_id"
    provider_models }o--|| provider_connections : "connection_id"
```
<!-- schemas-end -->