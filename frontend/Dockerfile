FROM node:20-alpine3.20 as builder

WORKDIR /frontend
COPY . .
RUN npm install
RUN npm run build

FROM httpd:2.4
COPY --from=builder /frontend/dist/ /usr/local/apache2/htdocs/
RUN ls /usr/local/apache2/htdocs