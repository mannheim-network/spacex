
## Building

### ⌨️ Build from source

#### 1. Install rust

> If, after installation, running `rustc --version` in the console fails, refer to [it](https://www.rust-lang.org/tools/install) to repair.

```shell
curl https://sh.rustup.rs -sSf | sh
```

#### 2. Initialize your wasm build environment

```shell
./scripts/init.sh
```

#### 3. Build wasm and native code

```bash
cargo build --release
```

#### *4. Troubleshooting

> Depending on different building environments, if you cannot build the source code, please check the detail error message and try to run the corresponding commands to fix it

- Debian/Ubuntu/Raspbian

```shell
sudo apt install gcc-multilib
wget https://apt.llvm.org/llvm.sh
sudo ./llvm 10
sudo ln -s /usr/lib/llvm-10/bin/llvm-config llvm-config
sudo apt install gcc
sudo apt install clang
```

- Fedora/RedHat/CentOS

```shell
sudo yum -y install gcc
sudo yum -y install clang
```

### 🐳 Dockerize

```
docker pull mannheimworld/spacex:rubik
docker run -v /tmp/spacex:/tmp/spacex --network host mannheimworld/spacex:rubik   ./spacex --base-path /tmp/chain --chain rubik [more_options]
```
## ⛰ Live Network

### 1. Connect to rubik

> The default branch `rubik` can be build and connect to rubik.

```shell
./target/release/spacex --chain rubik
```

### 2. Run as dev

Purge any existing developer chain state:

```bash
./target/release/spacex purge-chain --dev
```

Start a development chain with:

```bash
./target/release/spacex --dev
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev`.

### 3. Run as local

If you want to see the multi-node consensus algorithm in action locally, then you can create a local testnet with two validator nodes for Alice and Bob, who are the initial authorities of the genesis chain that have been endowed with testnet units.

You'll need two terminal windows open.

We'll start Alice's substrate node first on default TCP port `30333` with her chain database stored locally at `/tmp/alice`. The bootnode ID of her node is `12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp`, which is generated from the `--node-key` value that we specify below:

```bash
./target/release/spacex \
  --base-path /tmp/alice \
  --chain local \
  --alice \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001
```

In the second terminal, we'll start Bob's substrate node on a different TCP port of `30334`, and with his chain database stored locally at `/tmp/bob`. We'll specify a value for the `--bootnodes` option that will connect his node to Alice's bootnode ID on TCP port 30333:

```bash
./target/release/spacex \
  --base-path /tmp/bob \
  --chain local \
  --bob \
  --port 30334 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

Additional CLI usage options are available and may be shown by running `cargo run -- --help`.

