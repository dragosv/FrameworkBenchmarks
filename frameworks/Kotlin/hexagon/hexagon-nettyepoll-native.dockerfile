#
# BUILD
#
FROM ghcr.io/graalvm/native-image-community:17-ol9 as build
USER root
WORKDIR /hexagon

ADD . .
RUN microdnf -y install findutils
RUN ./gradlew --quiet classes
RUN ./gradlew --quiet -x test nativeCompile

#
# RUNTIME
#
FROM scratch
ARG PROJECT=hexagon_nettyepoll_postgresql

COPY --from=build /hexagon/$PROJECT/build/native/nativeCompile/$PROJECT /

ENTRYPOINT [ "/hexagon_nettyepoll_postgresql" ]
