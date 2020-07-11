FROM rust:1.44-alpine as builder

LABEL rustupInstalled TRUE

RUN apk add --update curl
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN apk add --update nodejs npm
WORKDIR /opt/services/djangoapp/src
RUN apk add build-base
COPY . ./
RUN npm install

RUN npm rebuild node-sass
CMD [ "npm", "run-script", "development"]

