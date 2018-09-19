#/usr/bin/env sh

VERSION="0.3.2"

cargo clean
docker build -t deciduously-com .
docker tag deciduously-com deciduously0/deciduously-com:$VERSION
docker tag deciduously-com deciduously0/deciduously-com:latest
docker push deciduously0/deciduously-com:$VERSION
docker push deciduously0/deciduously-com:latest