---
name: ui-crud
description: Use this when building or updating CRUD screens and forms (typed routes, handlers, Daisy RSX UI, validator-based forms, and Clorinde SQL-only data access).
---

# Octo UI CRUD

Apply this workflow for new or changed UI CRUD features in this repo.

## Required Architecture

- Typed routes live in `crates/octo-ui/src/routes.rs` and follow `/o/{org_id}/...`.
- UI pages live under `crates/octo-ui/src/<feature>/`.
- Main page file is `page.rs`; additional pages use clear names like `new.rs`, `upsert.rs`.
- Handlers live under `crates/octo/src/handlers/<feature>/`.
- GET handlers go in `loaders.rs`.
- POST handlers go in `actions.rs` with function names prefixed `action_`.
- `mod.rs` re-exports loaders/actions.

## DB + Query Rules

- Never embed SQL in Rust.
- All SQL must live in `crates/db/queries/*.sql` and be executed via generated Clorinde functions.
- If query shape changes, update SQL and rely on generated types.

## Layout + Page Rules

- Use `Layout` with:
  - `header_left` for breadcrumbs
  - `header_right` for page actions/buttons
- Use `SectionIntroduction` near top of page.
- Default content width is provided by layout; only override with `content_class` when needed.

## Form Rules

- Use `validator` (`derive(Validate)`) on form structs in handlers.
- Validate before DB calls.
- On validation failure:
  - Re-render the same page (do not return 422 for user input errors).
  - Show Daisy `Alert` with error message.
  - Preserve submitted values with a draft struct passed to UI.
- All form controls should be wrapped in `Fieldset` with:
  - `legend`
  - `help_text`

## Daisy RSX Component Reference (Prefer these)

- `Accordian`
- `Alert`, `AlertColor`
- `AppLayout`
- `Avatar`, `AvatarSize`, `AvatarType`
- `Badge`, `BadgeColor`, `BadgeSize`, `BadgeStyle`
- `BlankSlate`
- `Breadcrumb`, `BreadcrumbItem`
- `Button`, `ButtonScheme`, `ButtonShape`, `ButtonSize`, `ButtonStyle`, `ButtonType`
- `Card`, `CardBody`, `CardHeader`
- `CheckBox`, `CheckBoxScheme`, `CheckBoxSize`
- `Drawer`, `DrawerBody`, `DrawerFooter`
- `DropDown`, `DropDownLink`, `Direction`
- `Fieldset`
- `FileInput`, `FileInputColor`, `FileInputSize`, `FileInputStyle`
- `Input`, `InputSize`, `InputType`
- `SiteHeader`
- `Modal`, `ModalAction`, `ModalBody`
- `NavGroup`, `NavItem`, `NavSubGroup`, `NavSubItem`
- `Pagination`
- `Range`, `RangeColor`
- `RelativeTime`, `RelativeTimeFormat`
- `Select`, `SelectOption`, `SelectSize`
- `TabContainer`, `TabPanel`
- `TextArea`, `TextAreaSize`
- `TimeLine`, `TimeLineBadge`, `TimeLineBody`
- `Timeline`, `TimelineItem`, `TimelineStart`, `TimelineMiddle`, `TimelineEnd`
- `ToolTip`, `ToolTipColor`

## Local Shared Components

- `CardItem` (`crates/octo-ui/src/components/card_item.rs`)
- `SectionIntroduction` (`crates/octo-ui/src/components/section_introduction.rs`)

## Completion Checklist

- `cargo fmt`
- `cargo check --workspace`
- Handlers/routes/page wiring compile and route paths match typed routes.
