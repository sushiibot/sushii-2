import { UserModel } from '../../../zod-types';
import { z } from 'zod';

export const fromStoredUserModel = UserModel.extend({
  id: z.bigint().transform((x) => x.toString()),
  isPatron: z.boolean(),
  patronEmoji: z.string().nullish(),
  rep: z.bigint().transform((x) => x.toString()),
  fishies: z.bigint().transform((x) => x.toString()),
  lastRep: z
    .date()
    .transform((d) => d.getTime())
    .nullish(),
  lastFishies: z
    .date()
    .transform((d) => d.getTime())
    .nullish(),
});

export const TransportUser = UserModel.extend({
  id: z.string(),
  isPatron: z.boolean(),
  patronEmoji: z.string().nullish(),
  rep: z.string(),
  fishies: z.string(),
  lastRep: z.number().nullish(),
  lastFishies: z.number().nullish(),
});

export type TransportUserModel = z.infer<typeof TransportUser>;

export const fromTransportUserModel = UserModel.extend({
  id: z.string().transform(BigInt),
  isPatron: z.boolean(),
  patronEmoji: z.string().nullish(),
  rep: z.string().transform(BigInt),
  fishies: z.string().transform(BigInt),
  lastRep: z
    .string()
    .transform((ms) => new Date(ms))
    .nullish(),
  lastFishies: z
    .string()
    .transform((ms) => new Date(ms))
    .nullish(),
  lastfmUsername: z.string().nullish(),
});

export type StoredUserModel = z.infer<typeof fromTransportUserModel>;

export function getDefaultTransportUserModel(id: string): TransportUserModel {
  return {
    id,
    isPatron: false,
    rep: '0',
    fishies: '0',
    profileData: {},
  };
}
