FROM rust:1.28.0

WORKDIR /usr/src/deciduously-com
COPY . .

RUN cargo install

EXPOSE 80

CMD ["deciduously-com"]
