FROM nginx:1.25.2

RUN rm /etc/nginx/conf.d/default.conf

COPY ./nginx.no-bucket.conf /etc/nginx/conf.d/
