# kipi

A password manager.

### Fair Warning

If your plan is to seriously deploy this password manager for any reason, take serious precaution.
Despite the pride I take in my work here, there's better developed, FOSS alternatives, like [bitwarden](<https://bitwarden.com/>).

### Working Out of the Box

This repository provides a database out of the box using [Docker Compose](<https://docs.docker.com/compose/>).
Familiarity with Docker is necessary for deploying or testing this project at a bare minimum.

Commands of interest would be:

- `docker-compose start`
- `docker-compose stop`
- `docker ps -a`

All information regarding connecting to the container's PostgreSQL is available in [`.env`](.env).

```sh
docker-compose up -d
source .env
cargo run $DATABASE_URL

# once done...
docker-compose stop
```

