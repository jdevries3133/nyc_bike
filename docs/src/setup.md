# Setup

To get started with this boilerplate, follow these steps:

## Add Your Domains

Change domains of the frontend and backend in their respective `infra.tf`
files.

## Rename Docker Container & Account

Makefiles for the docs site, web front, and backend projects define the docker
account name and container names at the very top. Change these to your docker
account and the container names that make sense for your project.

## git tag Required

If you wipe out version control and start fresh (probably not a bad idea), make
sure you tag your initial commit (like `git tag v0.1.0`). Important identifiers
like docker image tags are derived from the git tag, so lots of things will
break if there are not any tags in version history.

## Setup GitHub Actions Credentials / Environment

Details are in [CI/CD Docs,](./ci_cd.md) but the CI pipeline needs some
credentials to send your code through to production.

### (optional) Modify GitHub Action Triggers

GitHub actions currently run for every commit on every branch. This is probably
not desirable and you should at least limit deployments to running from the
main branch, maybe only for tagged commits.
