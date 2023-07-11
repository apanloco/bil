FROM rust AS build

RUN rustup target add wasm32-unknown-unknown
WORKDIR /app
RUN mkdir /tmp/deleteme \
    && cd /tmp/deleteme \
    && cargo init \
    && cargo add macroquad \
    && rm -rf /tmp/deleteme
COPY ./ /app
RUN cargo build --target wasm32-unknown-unknown --release

FROM nginx:alpine

COPY index.html /usr/share/nginx/html/
COPY favicon.ico /usr/share/nginx/html/
COPY assets /usr/share/nginx/html/assets
COPY --from=build /app/target/wasm32-unknown-unknown/release/bil.wasm /usr/share/nginx/html/

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

