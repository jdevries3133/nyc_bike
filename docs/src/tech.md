# Tech Choices

## All Around

All infrastructure is going to Kubernetes via Docker and Terraform. The backend
app container and database (postgres) will run in Kubernetes, the frontend
static site will be served from Kubernetes

## Web Frontend

- [yew](https://yew.rs/)
- [tailwind CSS](https://tailwindcss.com/)

## Backend

- [rocket](https://rocket.rs/)
- [diesel](https://diesel.rs/)
