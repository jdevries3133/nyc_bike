# Rust App Template

Template for a full stack rust app, including:

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

## Documentation Website

- [mkdocs](https://www.mkdocs.org/)

## Docker / Terraform / Kubernetes

The documentation site and web backend are deployed to my Kubernetes cluster,
but should be portable to any Kubernetes with minor adjustments for your
platform.

[Read more about my dev workflow targeting my cluster.](https://jackdevries.com/blog/developingClusterApps)

[Read more about how I built my cluster.](https://jackdevries.com/blog/myCluster)
