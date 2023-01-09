FROM ubuntu:latest

RUN apt-get update
#RUN apt-get install libc6-i386

WORKDIR /usr/src/beam

# Copy the executable file from the target/debug directory into the Docker image
COPY ./target/debug/beam /usr/local/bin/

# Run the executable file when the Docker container starts
CMD ["beam"]
EXPOSE 8000