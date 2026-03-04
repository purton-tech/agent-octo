# Rules and Guidelines

This is a [Rust on Nails](https://rust-on-nails.com/) project using Rust to build a full stack web application.

You are running in a `devcontainer`. The developer is running a handful of `watch` commands from the `Justfile` for code generation.

## Tech Stack

* Axum              # Handles all the applications routes and actions https://github.com/tokio-rs/axum
* Clorinde          # Generates a Rust crate from `.sql` files with type-checked Postgres queries https://halcyonnouveau.github.io/clorinde/
* Dioxus rsx! macro # Used to create UI components and pages on the server side. https://dioxuslabs.com/
* Daisy UI          # Tailwind components https://daisyui.com/
* daisy_rsx         # A rust crate that implements the Daisy UI components in rsx!
* DbMate            # Database Migrations https://github.com/amacneil/dbmate
* Postgres          # Database
* Earthly           # Build system for production. https://earthly.dev/

## Database Migrations

* `dbmate` will only work if you replicate the operation of the `dbmate()` function in `bash_aliases`.
* When adding a new enum value (e.g., `ALTER TYPE ... ADD VALUE`), do not use the new value in the same migration transaction. Split into a follow-up migration before inserting rows that reference the new enum value.

## Folder: db

* All of the `dbmate` migrations are stored in the `migrations` folder.
* To create a new migration run `dbmate new migration-name` where migration name somehow represents the work you are doing. Always use `dbmate new` so timestamps are correct.
* All of the `.sql` files are in a folder called `queries`.
* The `sql` files are named after the main tables or schemas. i.e. `users.sql` for the `users` table.
* All the database CRUD operation are in these files.
* When you update the file a code generator runs and creates rust code from the sql. (clorinde).
* Clorinde generates a dedicated Rust crate which is re-exported from `crates/db/lib.rs`.

### Clorinde SQL Guidelines

* **Struct Definitions**: Add `--: StructName` before queries to define return types
* **Query Naming**: Use `--! query_name` to name queries
* **Parameters**: Parameters are inferred automatically; do not declare them manually
* **Intervals**: Use `($1 || ' days')::INTERVAL` for dynamic intervals
* **Optional Fields**: Use `field_name?` for nullable fields when required

## Folder: web-assets (sometimes a different name in different projects)

* Any images that are needed by the application are stored in a sub folder called images
* Also the tailwind config is stored here.
* The user will run `just tailwind` this will watch the tailwind `input.css` and src files for any changes.
* When changes occur the resulting `tailwind.css` is stored in a `dist` folder.
* There is a `build.rs` it uses a crate called `cache-busters` that sees the images and css files.
* It takes the hash of the files and crates a struct that gives us the ability to access the images by name in a typesafe way.
* For example the `tailwind.css` will be exported as `web_assets::files::tailwind_css` in the app and we reference it by calling `web_assets::files::tailwind.name`.

## Folder: web-pages (sometimes a different name in different projects)

* Every route has its own folder under `crates/web-pages`.
* The main page for a route lives in a file called `page.rs` inside that folder.
* Additional components are stored either alongside `page.rs` or in a `components/` folder.
* Shared widgets such as confirmation dialogs live under `components/` at the crate root.
* Each page corresponds to a typed route defined in `crates/web-pages/routes.rs` and is called from the matching handler in `crates/web-server/handlers`.
* We use Tailwind and Daisy UI. Only use Daisy UI colors and when possible the provided Daisy RSX library.
* Buttons can open modals by setting `popover_target` to the modal's `trigger_id`.

## Folder: web-server (sometimes a different name in different projects)

* Every route lives in its own folder under `crates/web-server/handlers`.
* GET endpoints are implemented in `loader.rs`.
* POST endpoints are implemented in `actions.rs` with functions prefixed by `action_`.
* `mod.rs` re-exports the loader and actions and defines the `routes()` helper used by `main.rs`.
* Each loader function fetches data from the database and renders the page.
* Actions call the appropriate database functions before redirecting the browser.
## Running the unit tests

* Use `just test` or `cargo test --workspace --exclude integration-testing`
* This will exclude the integration-testing which requires an environment with selenium.
