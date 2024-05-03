FROM golang:1.21@sha256:cffaba795c36f07e372c7191b35ceaae114d74c31c3763d442982e3a4df3b39e

ARG DATABASE_URL="host=localhost:5432 user=postgres password= dbname=postgres port=5434 sslmode=disable TimeZone=Europe/Oslo"
ARG SECRET="SECRET"

ENV DATABASE_URL=${DATABASE_URL} \
    SECRET=${SECRET}

LABEL org.opencontainers.image.source="https://github.com/martinellegard/smarter-home"

# Enviroment variable
WORKDIR /usr/src/smarter-home

RUN go install github.com/cosmtrek/air@latest

#Copying files to work directory
COPY go.mod ./
RUN go mod download && go mod verify
COPY . .

# Run and expose the server on port 3000
EXPOSE 3000

ENTRYPOINT ["air"]
# CMD [ "nodemon", "build/app.js" ]
