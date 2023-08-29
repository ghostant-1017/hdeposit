cargo run --release -- --contract 0x5469317FE035b3b0C70017726b6f98478e98325d \
--eth1-endpoint https://eth.getblock.io/310a66fb-9df2-4436-a22f-b7d7d28092e9/goerli/ \
--eth2-endpoint http://localhost:5052/ \
--dsn "postgresql://postgres:postgres@127.0.0.1/postgres?connect_timeout=10" \
--password Ipfs@111 \
--chain-id 1 \
--start 9577248