FROM nginx:stable-alpine

RUN rm /etc/nginx/conf.d/*
COPY ./default.conf /etc/nginx/conf.d/
COPY ./shared_location.conf /etc/nginx/

EXPOSE 80

RUN /bin/sh -c 'while :; do sleep 6h & wait $${!}; nginx -s reload; done & nginx -g \"daemon off;\"'
