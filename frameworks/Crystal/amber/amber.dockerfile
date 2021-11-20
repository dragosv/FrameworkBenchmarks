FROM crystallang/crystal:1.10.1-alpine as build

WORKDIR /amber
COPY config config
COPY src src

COPY shard.lock shard.lock
COPY shard.yml shard.yml

RUN apk update 
RUN apk add yaml-dev sqlite-dev

# RUN shards build amber --release
RUN shards install
RUN crystal build --release --no-debug --static src/amber.cr
RUN cp amber bin/amber

# Main Image
FROM ubuntu:22.04

WORKDIR /amber

ENV AMBER_ENV production
ENV DATABASE_URI postgres://benchmarkdbuser:benchmarkdbpass@tfb-database:5432/hello_world?initial_pool_size=56&max_idle_pool_size=56&max_pool_size=112

RUN apt update 
RUN apt install -yqq libyaml-dev

COPY run.sh run.sh
RUN mkdir bin
COPY --from=build /amber/amber /amber/bin/amber

EXPOSE 8080

CMD bash run.sh

