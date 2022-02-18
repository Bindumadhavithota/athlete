import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { connect, sendTransactionAsync } from "../src/utils";
import { Enum, Option, u32 } from "@polkadot/types-codec"
import type { AccountId32, H256 } from '@polkadot/types/interfaces/runtime';

import { MetaAthletePrimitivesCard } from "@polkadot/types/lookup";

function getRandomInt(min: number, max: number): number {
  min = Math.ceil(min);
  max = Math.floor(max);
  return Math.floor(Math.random() * (max - min + 1)) + min;
}

function printCard(athlete: string, hex: H256, card: MetaAthletePrimitivesCard) {
  console.log(`Card from onchain for ${athlete}: (${hex.toHex()}): tier=${card.tier.toString()} value=${card.value.toNumber() / 1_000_000_000_000} ATHL`);
}

function printGetCard( athlete: string, id: number, cards: Map<number, [H256, MetaAthletePrimitivesCard]>) {
  let card = cards.get(id)!;
  printCard(athlete, card[0], card[1]);
}

(async () => {
  let { api, alice } = await connect();


  api.registerTypes({
    AthleteCardClass: {
      _enum: ['Gold', 'Silver', 'Diamond']
    }
  })

  const athletes = await api.query.athletes.athletes.entries();
  const cards = await api.query.athletes.cards.entries();

  for (const entry of athletes) {
    if (entry[1].isSome) {
      const athlete = entry[1].unwrap();
      const athleteId = entry[0].args[0];
      const name = athlete.name.toUtf8();

      const height = Number(athlete.height.millimeters.toBigInt()) / 10;
      const weight = Number(athlete.weight.grams.toBigInt()) / 1000;
      console.log(`Registered athlete ${name}: height=${height}cm weight=${weight}kg`)
      console.log(`Cards minted for ${name}: ${athlete.cardsMinted}`)

      let goldCards = 0;
      let silverCards = 0;
      let diamondCards = 0;

      const athleteCards: Map<number, [H256, MetaAthletePrimitivesCard]> = new Map();
      for (const entry of cards) {
        const key = entry[0].args[0];
        const opt = entry[1] as Option<MetaAthletePrimitivesCard>;
        if (opt.isSome) {
          const card = opt.unwrap();

          if (card.id.athleteId.toHuman() != athleteId.toHuman()) {
            continue
          }

          if (card.tier.isGold) {
            goldCards += 1;
          } else if (card.tier.isSilver) {
            silverCards += 1;
          } else if (card.tier.isDiamond) {
            diamondCards += 1;
          }

          athleteCards.set(card.id.instanceId.toNumber(), [key, card]);
        }
      }

      console.log(`Gold: ${goldCards}; Silver: ${silverCards}; Diamond: ${diamondCards};`);
      printGetCard(name, 1, athleteCards);
      printGetCard(name, 20, athleteCards);
      printGetCard(name, 110, athleteCards);

      console.log();
    }
  }
})()

