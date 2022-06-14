# Build System

Make is used as a build system. Each subproject at least has rules for
`develop` and `deploy`.

Since there are different CLI tools in different parts of this project, make is
used to create a higher-level abstraction over these tools, which include:

- cargo: rust build system
- trunk: WASM bundler
- docker: containers
- flutter: cross-platform native app builder (iOS and Android)

## Dependencies

Makefiles assume that you have installed these CLI tools.

- rust & cargo
- [trunk](https://trunkrs.dev/)
- [entr\*](https://eradman.com/entrproject/) (CLI tool to re-run commands based on file changes)
- [concurrently](https://www.npmjs.com/package/concurrently/v/6.5.1) (run `npm install --global concurrently`)
- [tailwindcss](https://www.npmjs.com/package/tailwindcss) (run `npm install --global tailwindcss`)
- [flutter](https://docs.flutter.dev/get-started/install)

_\*this is only required for development and need not be installed in CI/CD_

The CI/CD in `.github/workflows/ci_cd.yml` takes care of installing these
things up front.

## Top-Level Makefile

The top-level Makefile has rules to support testing, developing, and deploying
the whole system together, which dispatches to subproject rules, either
sequentially (`check`, `deploy`) or concurrently (`develop`).

### `make develop`

Concurrently runs the web frontend server, web backend server, postgres
database, and mobile app.

### `make deploy`

Push docker containers for all sub-projects. They will be tagged with the
output of `git describe --tags`, then deploy everything to Kubernetes with
Terraform.

Automated deployment of iOS and Android apps is not setup, so nothing is done
for that part of the project.

### `make check`

Run checks for all subprojects. As of now, this just runs unit tests, but I
will consider this boilerplate done when this rule also runs system integration
tests.
