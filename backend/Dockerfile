FROM scratch

WORKDIR /opt/rust_template_backend

# the backend executable is created and copied here by the `pre-docker-build`
# make rule
COPY backend .
COPY ./Rocket.toml .

CMD ["./target/release/backend"]
