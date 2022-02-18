import { ApiPromise } from "@polkadot/api";

/**
 * Athlete-specific information that is available on the blockchain.
 */
export interface Athlete {
  /**
   * Numeric id of the athlete. Can be used to refer to the athlete throughout the system.
   */
  id: string,
  /**
   * Athlete name.
   */
  name: string,
  /**
   * Athlete weight in grams.
   */
  weight: number,
  /**
   * Athlete height in millimeters.
   */
  height: number,
  /**
   * A Blake2 hash of the athlete's photo, if present.
   * The actual photo isn't stored on the chain, just the hash.
   */
  photo?: string,
}

export enum CardClass {
  Gold,
  Silver,
  Diamond,
}

export interface AthleteCard {
  athleteId: string,
  cardId: string,
  class: CardClass,
  hash: string,
}

/**
 * Get all athletes from the chain.
 *
 * @param api Handle to the Substrate-based API entrypoint
 * @returns A list of all athletes
 */
export async function getAllAthletes(api: ApiPromise): Promise<Athlete[]> {
  return null as any
}

/**
 * Get all approved athletes from the chain.
 * This doesn't include athletes that have submitted applications but weren't approved yet.
 *
 * @param api Handle to the Substrate-based API entrypoint
 * @returns A list of all approved athletes
 */
export async function getApprovedAthletes(api: ApiPromise): Promise<Athlete[]> {
  return null as any
}

/**
 * Get all applicant athletes from the chain.
 * This only includes athletes that have submitted applications, but weren't approved yet.
 *
 * @param api Handle to the Substrate-based API entrypoint
 * @returns A list of all approved athletes
 */
export async function getApplicantAthletes(api: ApiPromise): Promise<Athlete[]> {
  return null as any
}

/**
 * Get all cards minted for a specific athlete.
 *
 * @param api
 * @param athleteId
 * @returns
 */
export async function getAthleteCards(api: ApiPromise, athleteId: string): Promise<AthleteCard[]> {
  return null as any
}
