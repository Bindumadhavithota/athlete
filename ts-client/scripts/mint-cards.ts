import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { connect, sendTransactionAsync } from "../src/utils";

import "./interfaces/augment-api";

(async () => {
  let { api, alice } = await connect();

  const athletes = await api.query.athletes.athletes.entries();
  for (const entry of athletes) {
    if (entry[1].isSome) {
      const athlete = entry[1].unwrap();
      const id = entry[0].args[0];
      if (!athlete.cardsMinted.toHuman()) {
        const tx = api.tx.athletes.mintCards(id);
        await sendTransactionAsync(api, alice, tx, `mint cards for ${athlete.name.toUtf8()}`);
      } else {
        console.log(`skipping ${athlete.name.toUtf8()} because they already have cards minted`);
      }
    }
  }
})()

