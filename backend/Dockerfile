FROM debian:buster-slim

RUN adduser --disabled-password --no-create-home rocket
USER rocket

WORKDIR /app

COPY ./Rocket.toml .
COPY ./backend .

CMD ["./backend"]
