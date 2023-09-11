cargo run --release -- --eth2-endpoint http://localhost:5052/ \
--eth1-endpoint https://stylish-soft-shadow.ethereum-goerli.discover.quiknode.pro/0ee6b1dcfb32c48a5ad26f4ff7157a26e1bc7537/
--dsn "postgresql://postgres:postgres@127.0.0.1/postgres?connect_timeout=10" \
--socket 0.0.0.0:11111
--start 6418923