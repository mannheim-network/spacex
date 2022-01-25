# Build spacex image
FROM ubuntu:20.04

RUN apt-get update
RUN apt-get install -y openssl
COPY spacex /opt/spacex/spacex
COPY run.sh /opt/run.sh

WORKDIR /opt/spacex/
CMD /opt/run.sh
