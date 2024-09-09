# this dockerfile is made for using in deployment, in development is better to use node installed localy
FROM node:20-alpine3.20 AS builder

WORKDIR /frontend
ARG IS_DEV
COPY . .
RUN npm install
RUN sh build.sh


FROM nginx:1.27-alpine
COPY --from=builder /frontend/dist/ /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf