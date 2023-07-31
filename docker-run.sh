sudo docker run \
  --name SCAICT-Website-API \
  --detach \
  --publish 80:80 \
  --publish 443:443 \
  --env-file .env \
  --volume /etc/letsencrypt/archive/api.scaict.org:/etc/letsencrypt/archive/api.scaict.org:ro \
  --volume /etc/letsencrypt/live/api.scaict.org/cert.pem:/etc/letsencrypt/live/api.scaict.org/cert.pem:ro \
  --volume /etc/letsencrypt/live/api.scaict.org/privkey.pem:/etc/letsencrypt/live/api.scaict.org/privkey.pem:ro \
  scaict-website-api
