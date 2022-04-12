aura 1:
subkey generate
Secret phrase:       gain decorate hybrid thunder leaf volume brick garlic reject ten swing cloud
  Network ID:        substrate
  Secret seed:       0xa6b60fc66a59f3dfdb6a06608eb36422573134408ed6e30b69396de61cb9ae81
  Public key (hex):  0xec675ffd1662b0b5a5ad65fdbbd8f75fe4dc256ee8f1d23fb93426e9cea2c54c
  Account ID:        0xec675ffd1662b0b5a5ad65fdbbd8f75fe4dc256ee8f1d23fb93426e9cea2c54c
  Public key (SS58): 5HQfsDvTxGovKFkjuLCLRjJMtyLA7eUcyZiGXgLh1yzxV4KS
  SS58 Address:      5HQfsDvTxGovKFkjuLCLRjJMtyLA7eUcyZiGXgLh1yzxV4KS

grandpa 1:
subkey inspect --scheme ed25519 "gain decorate hybrid thunder leaf volume brick garlic reject ten swing cloud"
Secret phrase:       gain decorate hybrid thunder leaf volume brick garlic reject ten swing cloud
  Network ID:        substrate
  Secret seed:       0xa6b60fc66a59f3dfdb6a06608eb36422573134408ed6e30b69396de61cb9ae81
  Public key (hex):  0xa1bd598cd43ccba380b2caa57d42f145394143f21401aa015f26a120bc9dc21a
  Account ID:        0xa1bd598cd43ccba380b2caa57d42f145394143f21401aa015f26a120bc9dc21a
  Public key (SS58): 5FimpEiaKTwm1rxf1f1E9nibGtLWJSvoPC1Wem9Q8MhR6WtL
  SS58 Address:      5FimpEiaKTwm1rxf1f1E9nibGtLWJSvoPC1Wem9Q8MhR6WtL


  aura 2: 
  subkey generate
Secret phrase:       need runway thumb shy exhaust tray sustain bridge accident chase high attend
  Network ID:        substrate
  Secret seed:       0xd6e1ff11dea7057e8efb7766ae8a9d637d76be203bc87fbe386e3399facb6def
  Public key (hex):  0x14d1c4362491686fda5a806cc45d9c9b8bf33da7f3eda33a0abe693126dd3656
  Account ID:        0x14d1c4362491686fda5a806cc45d9c9b8bf33da7f3eda33a0abe693126dd3656
  Public key (SS58): 5CY18QYoEPyiJ6wwDz5febFUYzoyr2HXqt9oQExTLEGDiyLP
  SS58 Address:      5CY18QYoEPyiJ6wwDz5febFUYzoyr2HXqt9oQExTLEGDiyLP
  
  grandpa 2:
  subkey inspect --scheme ed25519 "need runway thumb shy exhaust tray sustain bridge accident chase high attend"
Secret phrase:       need runway thumb shy exhaust tray sustain bridge accident chase high attend
  Network ID:        substrate
  Secret seed:       0xd6e1ff11dea7057e8efb7766ae8a9d637d76be203bc87fbe386e3399facb6def
  Public key (hex):  0xcb5651f247bbdec871ad0bb3bf6822c396250f6b4129ccc4ce4170205979c32b
  Account ID:        0xcb5651f247bbdec871ad0bb3bf6822c396250f6b4129ccc4ce4170205979c32b
  Public key (SS58): 5GfKDiwqrwMxC8a1D3r9BGWBVL79B6tNafPvFQ9DfNC2mRNa
  SS58 Address:      5GfKDiwqrwMxC8a1D3r9BGWBVL79B6tNafPvFQ9DfNC2mRNa

sudo account
subkey generate
Secret phrase:       senior marine eternal between afraid absorb bag soap yellow found ripple have
  Network ID:        substrate
  Secret seed:       0xbd60e090bf6ce561e32b4c4b9849ac7b4d2f3e4bb02d0da92635a1e21b7c0eed
  Public key (hex):  0x8aedf6237f03b7aa80b16e19078b55a96dfdf9f2ac12046b590676412f0b3e17
  Account ID:        0x8aedf6237f03b7aa80b16e19078b55a96dfdf9f2ac12046b590676412f0b3e17
  Public key (SS58): 5FCs9vExyWTMFL1GERiXstqyKgX8L99opRsFP4oU5WTs4L9s
  SS58 Address:      5FCs9vExyWTMFL1GERiXstqyKgX8L99opRsFP4oU5WTs4L9

validator 1
  ./target/release/node-template \
--base-path ./data/node01 \
--chain ./customSpecRaw.json \
--port 30333 \
--ws-port 9945 \
--rpc-port 9933 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--rpc-methods Unsafe \
--name MyNode01 \
--node-key 6d1e1dba1d7c86ca944ed9fb5000fd085ba5959e1a5978c370cad6f8feb7f4bd


validator 2
./target/release/node-template \
--base-path ./data/node02 \
--chain ./customSpecRaw.json \
--port 30334 \
--ws-port 9946 \
--rpc-port 9934 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--rpc-methods Unsafe \
--name MyNode02 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWGgXk7JgXwPUe9LVn3gDvMqqhupteMHfbzAs7qcvHTk3P


node key

subkey generate-node-key --file node-key
12D3KooWGgXk7JgXwPUe9LVn3gDvMqqhupteMHfbzAs7qcvHTk3P