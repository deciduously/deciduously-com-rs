#/usr/bin/env sh

VERSION="0.2.1"

cargo clean
docker build -t deciduously-com .
docker tag deciduously-com deciduously0/deciduously-com:$VERSION
docker push deciduously0/deciduously-com:$VERSION