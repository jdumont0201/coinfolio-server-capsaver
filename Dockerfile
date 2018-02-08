FROM ubuntu:16.04
MAINTAINER J. Dumont <j.dumont@coinamics.io>
RUN apt-get update && apt-get install -y libssl-dev pkg-config ca-certificates
ENV appname server-capsaver




RUN mkdir -p /coinfolio && mkdir /coinfolio/${appname}
ADD target/release/server-capsaver /coinfolio/${appname}
RUN chmod 777 /coinfolio/${appname}/server-capsaver 

CMD exec /coinfolio/${appname}/server-capsaver ${pairs}