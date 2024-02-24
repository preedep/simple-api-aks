FROM alpine:3.18.6
RUN apk add openssl
RUN apk --no-cache add ca-certificates \
    && update-ca-certificates \
    && rm -rf /var/cache/apk/*

RUN openssl s_client -connect southeastasia-1.in.applicationinsights.azure.com:443 -showcerts </dev/null 2>/dev/null | sed -e '/-----BEGIN/,/-----END/!d' | tee "/usr/local/share/ca-certificates/ca.crt" >/dev/null && \
update-ca-certificates \

RUN mkdir /app
WORKDIR /app
ADD ./target/x86_64-unknown-linux-musl/release/simple-api-aks /app/simple-api-aks
EXPOSE 8888
ENTRYPOINT ["/app/simple-api-aks"]
