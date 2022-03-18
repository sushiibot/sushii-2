import {
  APIChatInputApplicationCommandInteraction,
  APIUser,
} from "discord-api-types/v9";
import Context from "../../context";
import logger from "../../logger";

/**
 * Get inclusive random number between min and max
 *
 * @param min
 * @param max
 * @returns
 */
function getRandomInt(min: number, max: number): number {
  const ceilMin = Math.ceil(min);
  const floorMax = Math.floor(max);
  return Math.floor(Math.random() * (floorMax - ceilMin + 1)) + ceilMin;
}

/**
 * Get a random number from a normal distribution
 * Derived from https://stackoverflow.com/a/49434653
 *
 * @param min Minimum value
 * @param max Maximum value
 * @param skew Skew of distribution, 1 for normal distribution, < 1 to skew
 * left, > 1 to skew right
 * @returns number
 */
function randDistNumber(min: number, max: number, skew: number): number {
  let u = 0;
  let v = 0;

  // Convert [0,1) to (0,1)
  while (u === 0) {
    u = Math.random();
  }
  while (v === 0) {
    v = Math.random();
  }

  let num = Math.sqrt(-2.0 * Math.log(u)) * Math.cos(2.0 * Math.PI * v);

  num = num / 10.0 + 0.5; // Translate to 0 -> 1
  if (num > 1 || num < 0) {
    num = randDistNumber(min, max, skew);
  } else {
    // resample between 0 and 1 if out of range
    num **= skew; // Skew
    num *= max - min; // Stretch to fill range
    num += min; // offset to min
  }

  return num;
}

/**
 * Fishy types, lower index fishies are more common and should be worth less
 */
export enum CatchableType {
  Anchovy = "anchovy",
  Salmon = "salmon",
  AtlanticSalmon = "atlantic salmon",
  Tuna = "tuna",
  Halibut = "halibut",
  SeaBass = "sea bass",
  YellowfinTuna = "yellow tuna",
  PufferFish = "puffer",
  WildKingSalmon = "wild king salmon",
  SwordFish = "swordfish", // fish appended to end of name
  BluefinTuna = "bluefin tuna",
  // Constant probability catchable types
  Seaweed = "seaweed",
  Algae = "algae",
  // Special fishy types with custom rarities
  Golden = "golden",
  Rotten = "rotten",
  MrsPuff = "Mrs. Puff", // sorry Mrs. Puff
  RustySpoon = "rusty spoon",
}

const scaledTypes = [
  CatchableType.Anchovy,
  CatchableType.Salmon,
  CatchableType.AtlanticSalmon,
  CatchableType.Tuna,
  CatchableType.Halibut,
  CatchableType.SeaBass,
  CatchableType.YellowfinTuna,
  CatchableType.PufferFish,
  CatchableType.WildKingSalmon,
  CatchableType.SwordFish,
  CatchableType.BluefinTuna,
];

const normalTypes = [CatchableType.Seaweed, CatchableType.Algae];

const rareTypes = [
  CatchableType.Golden,
  CatchableType.Rotten,
  CatchableType.MrsPuff,
  CatchableType.RustySpoon,
];

/**
 * Gets a random fishy type, skewed towards lower indexed types as common
 *
 * @returns random FishyType
 */
function getRandomCatchable(): CatchableType {
  // Check fixed probability types
  const randInt = getRandomInt(0, 100);
  if (randInt < rareTypes.length) {
    return rareTypes[randInt];
  }

  // Check normal types
  if (randInt < normalTypes.length + rareTypes.length) {
    return normalTypes[randInt % normalTypes.length];
  }

  // Scaled types - Lower index fishies more common
  const idx = Math.floor(randDistNumber(0, scaledTypes.length, 3));
  return scaledTypes[idx];
}

export interface FishyValueRange {
  min: number;
  max: number;
  skew: number;
}

function getFishyValueRange(catchable: CatchableType): FishyValueRange {
  // Exhaustive switch statement
  // eslint-disable-next-line default-case
  switch (catchable) {
    case CatchableType.Anchovy:
      return { min: 5, max: 10, skew: 3 };
    case CatchableType.Salmon:
      return { min: 15, max: 30, skew: 3 };
    case CatchableType.Halibut:
      return { min: 10, max: 30, skew: 3 };
    case CatchableType.AtlanticSalmon:
      return { min: 20, max: 25, skew: 3 };
    case CatchableType.Tuna:
      return { min: 30, max: 40, skew: 3 };
    case CatchableType.SeaBass:
      return { min: 40, max: 50, skew: 3 };
    case CatchableType.YellowfinTuna:
      return { min: 40, max: 50, skew: 3 };
    case CatchableType.PufferFish:
      return { min: 20, max: 30, skew: 3 };
    case CatchableType.WildKingSalmon:
      return { min: 20, max: 30, skew: 3 };
    case CatchableType.SwordFish:
      return { min: 40, max: 60, skew: 3 };
    case CatchableType.BluefinTuna:
      return { min: 40, max: 70, skew: 3 };
    case CatchableType.Seaweed:
      return { min: 8, max: 15, skew: 1 };
    case CatchableType.Algae:
      return { min: 1, max: 5, skew: 1 };
    case CatchableType.Golden:
      return { min: 100, max: 300, skew: 3 };
    case CatchableType.Rotten:
      return { min: 1, max: 5, skew: 3 };
    case CatchableType.MrsPuff:
      return { min: 50, max: 80, skew: 3 };
    case CatchableType.RustySpoon:
      return { min: 1, max: 2, skew: 1 };
  }
}

/**
 * Response value of caught fishy
 */
export interface FishyResponse {
  caughtAmount: number;
  caughtType: CatchableType;
  oldAmount: string;
  newAmount: string;
}

export async function fishyForUser(
  ctx: Context,
  _interaction: APIChatInputApplicationCommandInteraction,
  user: APIUser
): Promise<FishyResponse> {
  const dbUser = await ctx.sushiiAPI.getUser(user.id);
  logger.debug(dbUser, "before");

  // Get new fishy count
  const caughtType = getRandomCatchable();
  const valueRange = getFishyValueRange(caughtType);
  const caughtNum = Math.floor(
    randDistNumber(valueRange.min, valueRange.max, valueRange.skew)
  );

  const oldAmount = dbUser.fishies;

  const newFishies = BigInt(dbUser.fishies) + BigInt(caughtNum);

  // Update fishies in data
  dbUser.fishies = newFishies.toString();

  logger.debug(dbUser, "after");

  // Update user api
  // await ctx.api.updateUser(dbUser);

  await ctx.sushiiAPI.updateUser(dbUser);

  return {
    caughtAmount: caughtNum,
    oldAmount,
    newAmount: newFishies.toString(),
    caughtType,
  };
}
