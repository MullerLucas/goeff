FROM nginx:1.21.6

COPY config/nginx.conf /etc/nginx/nginx.conf
COPY config/sites-available/hellmut.conf /etc/nginx/sites-enabled/hellmut.conf

COPY config/proxy.conf /etc/nginx/proxy.conf
COPY config/fastcgi.conf /etc/nginx/fastcgi.conf
COPY config/conf/mime.types /etc/nginx/conf/mime.types

COPY config/certs/fullchain.pem /etc/nginx/certs/fullchain.pem
COPY config/certs/privkey.pem /etc/nginx/certs/privkey.pem
