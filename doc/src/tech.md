# Tech Choices

All infrastructure is going to Kubernetes via Docker and Terraform. The backend
app container and database (PostgreSQL) will run in Kubernetes, the frontend
static site will be served from Kubernetes.

## Web Frontend

- [yew](https://yew.rs/)
- [tailwind CSS](https://tailwindcss.com/)

## Mobile

See [mobile](./mobile.md) for details.

- [flutter](https://flutter.dev/multi-platform)
- call Rust via [flutter FFI](https://docs.flutter.dev/development/platform-integration/c-interop)

## Backend

- [rocket](https://rocket.rs/)
- [diesel](https://diesel.rs/)
