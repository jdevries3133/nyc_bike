# For documentation site container
FROM python:3.10-alpine as build

WORKDIR /app

COPY mkdocs.yml .
COPY requirements.txt .
RUN pip install -r requirements.txt
RUN mkdir docs
COPY src src
RUN mkdocs build

FROM nginx:1.21-alpine

WORKDIR /usr/share/nginx/html
COPY --from=build /app/site .

ENTRYPOINT ["nginx", "-g", "daemon off;"]
