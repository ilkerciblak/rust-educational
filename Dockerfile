FROM rust:latest

WORKDIR /usr/src/app/

COPY . /usr/src/app/

CMD ["/bin/bash"]