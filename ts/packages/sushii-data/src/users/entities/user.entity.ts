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

export type TransportUserModel = z.infer<typeof fromStoredUserModel>;

export const fromTransportUserModel = UserModel.extend({
  id: z.bigint(),
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
