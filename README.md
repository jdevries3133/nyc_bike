# Rust App Template

Template for a full stack rust app, including these features:

- boilerplate for a rocket/diesel web backend, yew frontend, and flutter mobile
  app (calling Rust code via FFI)
- GitHub actions for CI/CD
- documentation website, including starter documentation on the boilerplate
  itself
- portable infrastructure as code for Kubernetes implemented with Terraform

## Getting Started

Take a look at the [getting started guide.](./docs/src/setup.md)

## Top-Level Tour

| Directory   | Contents                                 |
| ----------- | ---------------------------------------- |
| `backend`   | web backend with rocket and diesel ORM   |
| `doc`       | mkdocs documentation site                |
| `lib`       | shared library between all rust projects |
| `mobile`    | flutter project for iOS and Android apps |
| `web_front` | web frontend with Yew and Tailwind CSS   |

## Live App

- [frontend](https://nyc-bike.jackdevries.com)
- [backend](https://nyc-bike-back.jackdevries.com)
- [documentation site](https://nyc-bike-doc.jackdevries.com)
