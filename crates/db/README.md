## Database

This crate owns the application database schema and generated query layer.

We use:

- `dbmate` for schema migrations in [`migrations/`](migrations/)
- `clorinde` for generating typed Rust query code from SQL files in [`queries/`](queries/)
- `just -f crates/db/Justfile db-diagram` to refresh schema diagrams in this file from a live database

The diagram task expects `DATABASE_URL` to point at a database with the current migrations applied.

## Current Shape

The database is currently split across three schemas:

- `auth`: authentication identities and sessions
- `org`: multi-tenant organization data, membership rules, invitations, and helper functions used by RLS
- `public`: product tables used by the application

## Main Tables

The key tables today are:

- `auth.users`
- `auth.sessions`
- `org.orgs`
- `org.org_memberships`
- `org.org_invitations`
- `public.provider_connections`
- `public.provider_models`
- `public.agents`
- `public.conversations`
- `public.messages`
- `public.channels`
- `public.integrations`
- `public.integration_connections`

The `public` schema also contains shared enum types such as `resource_visibility`, `message_role`, `channel_type`, and `integration_auth_type`.

## Schema Diagrams

Run `just -f crates/db/Justfile db-diagram` to replace the generated diagrams below with fresh output from the current database.

<!-- schemas-start -->
### `auth`

Authentication users and sessions.

```mermaid
erDiagram
    users {
        timestamp_with_time_zone created_at 
        text email UK 
        text first_name 
        uuid id PK 
        text issuer UK 
        text last_name 
        text sub UK 
        timestamp_with_time_zone updated_at 
    }
```

### `org`

Organization tenancy, memberships, invitations, and org helper functions.

```mermaid
erDiagram
    org_invitations {
        uuid accepted_by_user_id FK 
        timestamp_with_time_zone created_at 
        character_varying email UK 
        timestamp_with_time_zone expires_at 
        uuid id PK 
        uuid invited_by_user_id FK 
        uuid org_id FK,UK 
        org_role role 
    }

    org_memberships {
        timestamp_with_time_zone joined_at 
        uuid org_id PK,FK 
        org_role role 
        uuid user_id PK,FK 
    }

    orgs {
        timestamp_with_time_zone created_at 
        uuid id PK 
        text name 
    }

    org_invitations }o--|| orgs : "org_id"
    org_memberships }o--|| orgs : "org_id"
```

### `public`

Application domain tables: providers, agents, conversations, channels, integrations, and shared enum types.

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

    channel_conversations {
        uuid channel_id FK,UK 
        uuid conversation_id FK,UK 
        timestamp_with_time_zone created_at 
        text external_conversation_id UK 
        text external_user_id 
        uuid id PK 
        text last_external_message_id 
        timestamp_with_time_zone updated_at 
    }

    channels {
        text bot_token 
        timestamp_with_time_zone created_at 
        uuid created_by_user_id FK 
        uuid default_agent_id FK 
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
        uuid channel_conversation_id FK 
        uuid channel_conversation_id FK 
        channel_message_direction channel_message_direction 
        channel_message_direction channel_message_direction 
        channel_message_status channel_message_status 
        channel_message_status channel_message_status 
        text content 
        uuid conversation_id FK 
        timestamp_with_time_zone created_at 
        text external_message_id 
        text external_message_id 
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
    channels }o--|| agents : "default_agent_id"
    conversations }o--|| agents : "agent_id"
    channel_conversations }o--|| channels : "channel_id"
    channel_conversations }o--|| conversations : "conversation_id"
    messages }o--|| channel_conversations : "channel_conversation_id"
    messages }o--|| conversations : "conversation_id"
    integration_connections }o--|| integrations : "integration_id"
    provider_models }o--|| provider_connections : "connection_id"
```
<!-- schemas-end -->

## Notes

- Most application tables are org-scoped and use row-level security policies built on `org.is_org_member(...)` and `org.is_org_admin(...)`.
- Resource-style tables use `resource_visibility` to distinguish `private` rows from `org`-shared rows.
- Secrets are stored outside the database; the schema stores secret references only.
