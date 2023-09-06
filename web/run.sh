cargo run --release -- --eth2-endpoint http://localhost:5052/ \
--dsn "postgresql://postgres:postgres@127.0.0.1/postgres?connect_timeout=10" \
--chain-id 1