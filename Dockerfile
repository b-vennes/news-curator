FROM rust:buster
LABEL authors="b-vennes"

COPY . /app
WORKDIR /app
RUN cargo build -r
RUN cp /app/target/release/news-curator /bin/news-curator
WORKDIR /
RUN rm -rf /app
