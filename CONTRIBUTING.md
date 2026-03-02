# Rules and Guidelines

This is a [Rust on Nails](https://rust-on-nails.com/) project using Rust to build a full stack web application.

## Tech Stack

* Axum              # Handles all the applications routes and actions https://github.com/tokio-rs/axum
* Clorinde          # Generates a Rust crate from `.sql` files with type-checked Postgres queries https://halcyonnouveau.github.io/clorinde/
* Dioxus rsx! macro # Used to create UI components and pages on the server side. https://dioxuslabs.com/
* Daisy UI          # Tailwind components https://daisyui.com/
* daisy_rsx         # A rust crate that implements the Daisy UI components in rsx!
* DbMate            # Database Migrations https://github.com/amacneil/dbmate
* Postgres          # Database
* Earthly           # Build system for production. https://earthly.dev/

## Setting up for Development

This project runs in a `devcontainer` and uses [k3d](https://k3d.io/stable/) to run supporting backend services i.e. Postgres.

1. Run `just dev-init` to setup `k3d`
2. Run `just dev-setup` to run the kubernetes operator that install Bionic into the locally running `k3d`.
3. If you get a *service unavailable* error wait a bit longer for *k3d* to start.
4. Once you see the message `Reconciliation successful.` you can CTRL+C the `just dev-setup`.
5. Use `k9s` to check the status of the services in `k3d`.
6. When all the services are loaded you can check by running `db` you should now have access to the database.
7. `dbmate up` to create the database tables
8. Run `wa` to build and watch the Bionic server.
9. In another terminal run `wp` to watch and compile the web assets i.e. JS and Css.
10. In another terminal run `wt` to watch and compile the tailwind assets.
11. You can now access the front end on `http://localhost:7703`.

## Running the integration tests

Read the docs in `crates/integration-testing/README.md` if that folder exists.

1. Run `just get-config` and `just selenium` to install selenium into `k3d`.
2. Replace the bionic pod with your local version `just md`.
3. Run the integration tests `cargo test -p integration-testing`.
4. You can monitor the integration tests via `NoVNC` at `http://localhost:7000` password `secret`.
