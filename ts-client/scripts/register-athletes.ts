import { ApiPromise, HttpProvider, Keyring, WsProvider } from "@polkadot/api";
import { AddressOrPair, ApiTypes, SubmittableExtrinsic } from "@polkadot/api/types";
import { Enum, u64 } from "@polkadot/types-codec";
import { MetaAthletePrimitivesAthleteApplication } from "@polkadot/types/lookup";
import { ISubmittableResult } from "@polkadot/types/types";
import { Athlete, TEST_ATHLETES } from "./athletes";
import { connect, sendTransactionAsync } from "../src/utils";

import "../src/interfaces/augment-api";

async function registerAthletes(api: ApiPromise, signer: AddressOrPair, athletes: Athlete[]) {
  // fetch existing athletes (registered and approved) to avoid registering duplicates
  const pending = await api.query.athletes.applications.entries();
  const approved = await api.query.athletes.athletes.entries();

  const existingAthletes: Set<string> = new Set();
  const approvedAthletes: Set<string> = new Set();
  const athleteIds: Map<string, u64> = new Map();

  for (const athlete of pending) {
    if (athlete[1].isSome) {
      // get the registered athlete's id and name
      const id = athlete[0].args[0];
      const name = athlete[1].unwrap().name.toUtf8();
      existingAthletes.add(name);
      athleteIds.set(name, id);
    }
  }

  for (const athlete of approved) {
    if (athlete[1].isSome) {
      // get the approved athlete's id and name
      const id = athlete[0].args[0];
      const name = athlete[1].unwrap().name.toUtf8();
      existingAthletes.add(name);
      approvedAthletes.add(name);
      athleteIds.set(name, id);
    }
  }

  for (const athlete of athletes) {
    // skip athletes that already exist as either approved or registered
    if (existingAthletes.has(athlete.name)) {
      console.log(`Skipping registration for ${athlete.name}`)
      continue;
    }

    console.log(`Registering athlete ${athlete.name}`);
    const registerResult = await sendTransactionAsync(api, signer, api.tx.athletes.submitAthleteApplication({
      name: athlete.name,
      applicantAccount: null,
      weight: { grams: athlete.weight * 1000},
      height: { millimeters: athlete.height * 1000},
      photo: null
    }), `register ${athlete.name}`);

    let id: u64 | undefined;
    for (const record of registerResult.events) {
      const event = record.event;
      if (api.events.athletes.AthleteApplicationSubmitted.is(event)) {
        id = event.data[0];
      }
    }

    if (!id) {
      throw "application submission failed";
    }
    athleteIds.set(athlete.name, id);
  }

  for (const athlete of athletes) {
    // skip athletes that are already approved
    if (approvedAthletes.has(athlete.name)) {
      console.log(`Skipping approval for ${athlete.name}`)
      continue;
    }

    console.log(`Approving athlete ${athlete.name}`);

    const id = athleteIds.get(athlete.name);
    if (!id) {
      throw `no id for athlete ${athlete.name}`
    }

    let approvalDone = false;
    const approvalResult = await sendTransactionAsync(api, signer, api.tx.sudo.sudo(api.tx.athletes.approveApplication(id)), `approve ${athlete.name}`);
    for (const record of approvalResult.events) {
      const event = record.event;
      if (api.events.athletes.AthleteApplicationApproved.is(event)) {
        approvalDone = true;
      }
    }

    if (!approvalDone) {
      throw "approval failed";
    }
  }
}

(async () => {
  let { api, alice } = await connect();

  await registerAthletes(api, alice, TEST_ATHLETES);
})()

