sudo podman run -dt -p 6379:6379 \
    --name di-redis \
    docker.io/redis:latest

sudo podman run -dt -p 80:1888 \
     --name di-api \
     -e ENV=PROD \
     -e JWT_SECRET=abel399 \
     -e SENTRY_SECURITY_TOKEN=4885702ed2d311ebb7f4026626ea21be \
     -e SENTRY_INGEST_URL=https://61cdc23d523c49a683f71a9c5ae01a6b@o558281.ingest.sentry.io/5827382 \
     -e DATABASE_URL=postgres://postgres:fuckshit01--__399@db.lqdzjmkossqcoaepyquw.supabase.co:5432/postgres \
     quay.io/devisa/devisa-api:latest

