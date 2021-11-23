FROM crystallang/crystal:1.2.2

WORKDIR /amber
COPY config config
COPY src src
COPY run.sh run.sh
COPY shard.lock shard.lock
COPY shard.yml shard.yml

ENV AMBER_ENV production
ENV DATABASE_URI postgres://benchmarkdbuser:benchmarkdbpass@tfb-database:5432/hello_world?initial_pool_size=56&max_pool_size=56&max_idle_pool_size=56

RUN apt-get install -yqq libyaml-dev
RUN shards build amber --release

EXPOSE 8080

CMD bash run.sh
