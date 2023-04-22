FROM rust:latest

RUN apt-get update && apt install -y openssl
WORKDIR /var/www/hellmut_server

CMD ["./goeff_server"]
