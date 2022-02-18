# 1. Prerequisites

1. Command line
2. `node` and `yarn` installed (basic JS development environment).
2.1. Run `npm install --global yarn` to install Yarn if needed.
3. Access to a MetaAthlete node (either locally or deployed somewhere)

Make sure to install all the necessary JS dependencies.

```
cd ts-client
yarn install
```

### 1.1 Building the node locally (if needed)

This step can be skipped if you're using an already deployed node.

The node can be built and run locally using Cargo. The `wasm32-unknown-unknown` Rust toolchain needs to be installed.

Then in the root of the project: `cargo build --release`

To run the node in dev mode: `target/release/meta-athlete-node`

# 2. Tweaking connection settings

Make sure the `NODE_ENDPOINT` constant in `ts-client/src/utils.ts` points to the correct node, e.g.:
```ts
const NODE_ENDPOINT = 'ws://127.0.0.1:9944'
```

or
```ts
const NODE_ENDPOINT = 'ws://137.184.98.120:9944'
```

### 2.1

You can also setup a Substrate explorer using Docker with this command:
```
docker run --rm -it --name polkadot-ui -e WS_URL=ws://137.184.98.120:9944 -p 9999:80 jacogr/polkadot-js-apps:latest
```

# 3. Tweaking uploaded data

The script by default exports 4 athletes in this format:
```ts
export const TEST_ATHLETES: Athlete[] = [
  {
    name: "John Doe",
    weight: 82.5,
    height: 1.80,
  },
  {
    name: "Bobby Smith",
    weight: 78,
    height: 1.83,
  },
  {
    name: "Kyle Abrams",
    weight: 75,
    height: 1.75,
  },
  {
    name: "David Simpson",
    weight: 85,
    height: 1.90,
  },
]
```

Edit `ts-client/src/athletes.ts` to change the list of athletes to be uploaded.

# 4. Registering the athletes

In the `ts-client` directory, run:
```
yarn athletes:register
```

This will:
1. Read athletes from `athletes.ts`
2. Submit a registration application for each athlete
3. Approve the registration for each athlete (placeholder for KYC process)

# 5. Minting the NFTs
```
yarn athletes:mint
```

This will mint 10 Diamond, 50 Silver and 100 Gold NFT cards for each athlete. The cards will be initially owned by the system account (i.e. no-one).

# 6. Viewing on-chain summary
```
yarn athletes:summary
```

This will display a short summary about each registered athlete, the number of cards available for this athlete, and a random card, pulled from the blockchain state.
