FROM almalinux:8-minimal

WORKDIR /app

COPY target/release/server /app/server

COPY configuration.yaml /app/configuration.yaml

COPY www/ /app/www

RUN chmod 755 server

EXPOSE 8000

CMD ["/app/server"]
