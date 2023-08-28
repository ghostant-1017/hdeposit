cargo run --release -- --contract 0x5469317FE035b3b0C70017726b6f98478e98325d \
--eth1_endpoint https://eth.getblock.io/310a66fb-9df2-4436-a22f-b7d7d28092e9/goerli/ \
--eth2_endpoint http://localhost:5052/ \
--dsn "postgresql://postgres:postgres@127.0.0.1/hellman?connect_timeout=10" \ 
--start 9577248