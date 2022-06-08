# Rust App Template

Template for a full stack rust app, including these features:

- boilerplate for a rocker/deisel web backend, yew frontend, and flutter mobile
  app (calling Rust code via FFI)
- GitHub actions for CI/CD
- documentation website, including starter documentation on the boilerplate
  itself
- portable infrastructure as code for Kubernetes implemented with Terraform

## Getting Started

Take a look at the [getting started guide.](./docs/src/setup.md)

## Technologies

### Web Frontend

- [yew](https://yew.rs/)
- [tailwind CSS](https://tailwindcss.com/)

### Mobile

See [mobile](./mobile.md) for details.

- [flutter](https://flutter.dev/multi-platform)
- call Rust via [flutter FFI](https://docs.flutter.dev/development/platform-integration/c-interop)

### Backend

- [rocket](https://rocket.rs/)
- [diesel](https://diesel.rs/)

### Documentation Website

- [mkdocs](https://www.mkdocs.org/)

### Docker / Terraform / Kubernetes

The documentation site and web backend are deployed to my Kubernetes cluster,
but should be portable to any Kubernetes with minor adjustments for your
platform.

[Read more about my dev workflow targeting my cluster.](https://jackdevries.com/blog/developingClusterApps)

[Read more about how I built my cluster.](https://jackdevries.com/blog/myCluster)
