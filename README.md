# Osmosis Indexer

This is a simple indexer for Osmosis. It is designed to be used with the [Osmosis API]()
and [Osmosis UI]().

## Requirements

- Rust
- PostgreSQL

### Install and Run Postgree

The easiest way to install Postgree is to use [Docker](https://www.docker.com/). The `docker-compose.yml` file in this repository can be used to run Postgree.

```bash
docker-compose up -d
```

This will start a Postgree instance on port `5432` with the following credentials:

- Username: `user`
- Password: `password`

And a database called `indexes`.

To access the database, you can use the following command:

```bash
docker exec -it <container-name> psql  -d indexes -U user
```
