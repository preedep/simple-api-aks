FROM alpine:3.19
RUN mkdir /app
WORKDIR /app
ADD ./target/x86_64-unknown-linux-musl/release/simple-api-aks /app/simple-api-aks
EXPOSE 8888
ENTRYPOINT ["/app/simple-api-aks"]
