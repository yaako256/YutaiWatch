# docker/prod/entrypoint.sh
#!/bin/sh

echo "start supercronic"

exec /usr/local/bin/supercronic /app/crontab