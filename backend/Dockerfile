FROM ubuntu:latest as user

RUN useradd -u 10001 appuser

FROM scratch

COPY --from=user /etc/passwd /etc/passwd

WORKDIR /app
COPY ./Rocket.toml ./backend ./

USER appuser

CMD ["./backend"]
