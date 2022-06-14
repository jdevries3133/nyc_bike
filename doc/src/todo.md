# To Do

This boilerplate is certainly incomplete, and with that in mind, here is a
wish list of things it could do, but I'm going to stop at a week of work on
this nonsense and not let the perfect be the enemy of the good. I'll also try
to cherry pick changes from the projects I build with this boilerplate and add
them here, so that some of these things get addressed with time. If you use
this boilerplate and address some of these pain points, please open a PR so
that it becomes part of the base project.

- [ ] github action should be less monolithic, should call check and deploy
      processes for each subproject in a separate job so that one bug doesn't kill
      the whole pipeline
- [ ] github action triggers can probably be a bit more intelligent while still
      being generic; everything runs on every push to any branch right now which is
      very extreme
- [ ] mobile project is somewhat incomplete; missing key features like testing
      in CI, automated deployment, and, most importantly, the whole flutter / rust
      / FFI vision is not implemented, so you can't call rust code from flutter
      which is a core idea of this project
- [ ] web frontend is using Client Side Rendering (CSR) which is _so_ 2016. I
      think SSR and hydration is still experimental and unreleased in Yew, though,
      but that should be added when the time comes, especially since the frontend is
      already going to Kubernetes / containers, so the compute environment is already
      there, it should be straightforward to change from CSR to SSR/SSG
- [ ] code quality checkers (linters, format checkers) should be part of CI
- [ ] there are no system integration tests in CI
- [ ] there is no system to integrate, overall...
  - [ ] web frontend does not call the backend
  - [ ] mobile frontend does not call the backend
  - [ ] backend does not query the database & database client lib isn't setup
