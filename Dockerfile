FROM rust:1.28.0

WORKDIR /usr/src/deciduously-com
ADD . /usr/src/deciduously-com

RUN cargo install

EXPOSE 80

CMD ["deciduously-com"]
