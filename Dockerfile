FROM debian:stable

WORKDIR /app

COPY target/release/server /app/server

COPY configuration.yaml /app/configuration.yaml

COPY www/ /app/www

COPY key.pem /app/key.pem

COPY cert.pem /app/cert.pem

RUN chmod 755 server

EXPOSE 8000

CMD ["/app/server"]
