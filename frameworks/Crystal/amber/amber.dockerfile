FROM crystallang/crystal:1.1.1

WORKDIR /amber
COPY config config
COPY src src
COPY run.sh run.sh
COPY shard.lock shard.lock
COPY shard.yml shard.yml

ENV GC_MARKERS 1
ENV AMBER_ENV production
ENV DATABASE_URI postgres://benchmarkdbuser:benchmarkdbpass@tfb-database:5432/hello_world?initial_pool_size=32&max_pool_size=32&max_idle_pool_size=32
#ENV DATABASE_URI mysql://benchmarkdbuser:benchmarkdbpass@tfb-database:3306/hello_world?initial_pool_size=32&max_pool_size=32&max_idle_pool_size=32

RUN apt-get install -yqq libyaml-dev
RUN shards build amber --release --no-debug

EXPOSE 8080

CMD bash run.sh
