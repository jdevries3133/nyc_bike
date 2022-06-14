# Continuous Integration and Delivery

Continuous integration and delivery via GitHub actions is included in this
project. Currently, there is a single monolithic pipeline, although I plan to
break it up so that smaller parts of the project can fail or proceed
independently, and things move faster overall.

## Credentials

The GitHub actions use an
["environment"](https://docs.github.com/en/actions/deployment/targeting-different-environments/using-environments-for-deployment)
as GitHub calls it. The environment is named `Kubernetes`, because this is the
main deployment target. The environment needs to be able to access terraform
remote state, which is currently kept in an S3 bucket, which is why aws access
keys and secret keys must be included. Then, the token for your docker account
is needed to push containers to docker hub. Of course, the whole Kubernetes
config for `kubectl` is also required to deploy to the cluster. So, these items
are required, overall:

- `AWS_ACCESS_KEY_ID`
- `AWS_SECRET_ACCESS_KEY`
- `DOCKERHUB_TOKEN`
- `KUBE_CONFIG`

## Documentation Site

CI/CD will call the `deploy` rule of the documentation site. This builds and
pushes the docker container to docker hub, then deploys the changes to
Kubernetes via terraform. The whole static site build process happens inside
the container in a two-stage build process. The final image is just the static
files copied into nginx.

## Web Front

The web frontend has a `setup-ci` rule which initializes terraform, downloads
`trunk`, and installs the wasm build tools with `rustup`. Then, the static site
is build outside of docker in the `pre-container-build` rule. The static files
are then copied into an Nginx container, pushed to docker hub, and deployed to
Kubernetes via Terraform.

## Backend

Again, there is a `setup-ci` rule, but in this case it just initializes
terraform. Since the CI server uses `x86_64` architecture, and my Kubernetes
cluster is too, the compilation is native and therefore very simple.
`cargo build --release` is ran, the binary is copied out of the parent folder,
the docker build process throws it into a minimal debian container, which is
shipped to docker hub and deployed to Kubernetes via Terraform.

## Mobile

Currently, there is no CI/CD for the mobile app. I have a lot to learn about
developing mobile apps, so this is left for later. However, it's just a flutter
boilerplate project for now.

At this point, I've put too much time into this boilerplate and need to just
move forward. I would like to make this part of the boilerplate stronger, and
will certainly accept pull requests to improve it.
