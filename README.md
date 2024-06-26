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
cargo run $DATABASE_URL # sourced from .env

# once done...
docker-compose stop
```

### Relying on a Different Database

The project as is assumes a fresh postgres user instance.
If you want to use your own database outside of the provided docker containers,
the process would be similar- just point the `kipi` instance to your database's URL and let loose.

```sh
cargo run $DATABASE_URL # own postgres URL
```

#### SQL Injection Challenge

Obviously, this project is mostly implemented for fun.
And, what's more fun than having your vault broken into,
and your keys stolen?
If you can find any relevant SQL injection tactics,
please make an [issue](<https://github.com/Borgerr/kipi/issues>).
