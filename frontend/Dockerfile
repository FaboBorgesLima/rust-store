# this dockerfile is made for using in deployment, in development is better to use node installed localy
FROM node:20-alpine3.20 as builder

WORKDIR /frontend
COPY . .
RUN npm install
RUN npm run build

FROM httpd:2.4
COPY --from=builder /frontend/dist/ /usr/local/apache2/htdocs/
COPY nginx.conf /etc/nginx/nginx.conf
RUN ls /usr/local/apache2/htdocs