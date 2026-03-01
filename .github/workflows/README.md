# Release Process

This repository uses two separate release flows:

- `Release Candidate`: builds and pushes an RC container image only
- `Release`: promotes a chosen RC image to a stable image tag and updates deployment manifests

## Release Candidate

Run the `Release Candidate` GitHub Actions workflow manually.

What it does:

- checks out `main`
- calculates the next semantic version using `semantic-release --dry-run`
- builds and pushes the RC image to GHCR

The RC image tag format is:

```text
ghcr.io/purton-tech/octo:<version>-rc.<github_run_number>
```

Example:

```text
ghcr.io/purton-tech/octo:1.2.0-rc.42
```

The workflow writes the final image tag into the GitHub Actions step summary.

This workflow does not create a git tag or a GitHub prerelease.

## Stable Release

After validating an RC image, run the `Release` GitHub Actions workflow manually.

What it does:

- takes an `rc_tag` input such as `1.2.0-rc.42`
- retags `ghcr.io/purton-tech/octo:<rc_tag>` to `ghcr.io/purton-tech/octo:<version>`
- creates the stable `v<version>` git tag from the current `main` commit
- updates image tags in the manifests under `infra-as-code/`
- commits those manifest updates if needed
- creates the GitHub Release for the stable tag

## Manual RC Release In GitHub

If you want a visible RC release entry in GitHub, create it manually in the GitHub Releases UI.

Use:

- a tag name like `v1.2.0-rc.42`
- the commit you validated
- the RC image produced by the `Release Candidate` workflow

## Notes

- RC version calculation always runs from `main`
- the stable release workflow promotes from RC image tags, not RC git tags
- the `Release` workflow expects an input like `1.2.0-rc.42`
- GitHub Actions may still be blocked from creating the stable git tag if the tagged commit includes workflow changes under `.github/workflows/`
- if that happens, let the workflow handle image promotion and manifest updates, then create the stable tag and GitHub Release manually
