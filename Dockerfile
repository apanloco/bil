FROM rust

RUN rustup target add wasm32-unknown-unknown
WORKDIR /app
COPY ./ /app
RUN cargo build --target wasm32-unknown-unknown --release

FROM nginx:alpine

COPY index.html /usr/share/nginx/html/
COPY favicon.ico /usr/share/nginx/html/
COPY nginx.conf /etc/nginx/nginx.conf
COPY assets /usr/share/nginx/html/assets
COPY --from=0 /app/target/wasm32-unknown-unknown/release/bil.wasm /usr/share/nginx/html/

EXPOSE 8080

CMD ["nginx", "-g", "daemon off;"]

