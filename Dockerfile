FROM rust:1.52
# RUN apk update && apk add bash && apk add yarn
# RUN apk add git
RUN rustup override set nightly
RUN mkdir /app
WORKDIR /app
COPY . /app
ENV DATABASE_URL DATABASE_URL
ENV SECRET_KEY SECRET_KEY
ENV ROCKET_DATABASES ROCKET_DATABASES
EXPOSE 8000
CMD [ "cargo", "run" ]