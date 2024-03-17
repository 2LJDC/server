FROM debian:12-slim

WORKDIR /app

COPY target/release/mini-server /app/server

COPY configuration.yaml /app/configuration.yaml

COPY www/* /app/www

RUN chmod 755 mini-server

EXPOSE 8000

CMD ["/app/mini-server"]
