FROM rust:latest as build

LABEL maintainer "country bumpkin software"
# ARG DB_USERNAME
# ARG DB_PASSWORD
# ARG DB_HOST
# ENV DATABASE_URL=postgresql://${DB_USERNAME}:${DB_PASSWORD}@${DB_HOST}/exposure_sites

ARG DATABASE_URL
RUN USER=root cargo new --bin corona-map
WORKDIR /corona-map
RUN apt install pkg-config
COPY ./Cargo.lock ./Cargo.lock 
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release \
    && rm src/*rs
COPY ./src src

RUN rm target/release/deps/* \
    && cargo build --release

FROM rust:slim-buster
RUN export DATABASE_URL=${DATABASE_URL}
COPY --from=build /corona-map/target/release/corona-map /
CMD ["./corona-map"]
