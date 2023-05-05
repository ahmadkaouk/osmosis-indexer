# Osmosis Blockchain Indexer

This is a Rust-based indexer for the Osmosis blockchain. The indexer connects to an Osmosis node, fetches block data and associated information, and stores it in a PostgreSQL database. Users can then query the database for various insights and analytics. The indexer also provides an HTTP API for easy access to the indexed data.

## Features

- Fetches block data from the Osmosis blockchain using the Tendermint RPC
- Stores block data in a PostgreSQL database
- Automatically tracks and fetches new blocks at regular intervals
- Resumes indexing from the latest height stored in the database
- Supports concurrent fetching of multiple blocks for better performance
- Reads configuration from a TOML file
- Provides a CLI interface with several options for customization
- Easy configuration and setup using Docker and environment variables
- HTTP API for easy access to the indexed data

## Prerequisites

- Rust (latest stable version)
- PostgreSQL (latest version)
- Docker (optional, for running PostgreSQL)

## Configuration

Create a `Config.toml` file in the project directory with the following structure:

```toml
[db]
url = "postgresql://user:password@localhost:5432/indexes"

[indexer]
rpc_url = "https://rpc.cosmos.directory/osmosis"
# Fetching interval in seconds
fetch_interval = 30
```

Replace the placeholders with the appropriate values for your setup, including the database connection string and the Osmosis RPC URL.

## CLI Usage

The indexer provides a command-line interface with several options for customization. The following is the help message:

```bash
Usage: osmosis-indexer [OPTIONS]

Options:
  -c, --config-path <CONFIG_PATH>  Path to the config file [default: Config.toml]
      --height <HEIGHT>            The height to start indexing from [default: 9479346]
  -h, --help                       Print help
  -V, --version                    Print version
```

## Setup

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/osmosis-blockchain-indexer.git
   cd osmosis-blockchain-indexer
   ```

2. Set up the PostgreSQL database:

   - Using Docker:
     Create a `docker-compose.yml` file as described in the previous sections, and run:

     ```bash
     docker-compose up -d
     ```

   - Without Docker:

     Install PostgreSQL on your system and create a new database and user.

3. Build and run the indexer:

   ```bash
   cargo build --release
   ./target/release/osmosis-indexer --height 9479346
   ```

   The indexer will start fetching data from the Osmosis blockchain and storing it in the PostgreSQL database. You can customize the behavior by providing the appropriate CLI options. In parallel, The API server will start and bind to `0.0.0.0:3000`.

## Querying the indexed data

With the indexer and API server running, you can now start querying the indexed data for insights and analytics. To do so, you can use any PostgreSQL client or library to connect to the database and issue SQL queries. You can also use the HTTP API provided by the indexer to easily access the indexed data using HTTP requests.

Here are some examples of how you might use the HTTP API:

1. To get the blocks proposed by a validator, issue an HTTP GET request to `/blocks/:validator`:

    ```bash
    curl http://127.0.0.1:3000/blocks/97AFE45395B74E784C88D45E5CCA2995019FAE08
    ```

    This will return a JSON array of block heights where the specified validator has proposed a block.

2. To get the total number of transactions in the last `N` blocks, issue an HTTP GET request to `/transactions/:last_blocks`:

    ```bash
    curl http://127.0.0.1:3000/transactions/5
    ```

    This will return a JSON number representing the total number of transactions in the last 5 blocks.

Remember that you can also directly query the PostgreSQL database for more advanced analytics and insights. The indexer stores the data in a well-structured schema, making it easy to write SQL queries to analyze the indexed data.
