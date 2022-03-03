import { GuildConfigModel } from '../../../zod-types';
import { z } from 'zod';

// GuildConfig model with bigints as strings. This is used for transport and by
// clients.
export const fromStoredGuildConfigModel = GuildConfigModel.extend({
  id: z.bigint().transform((x) => x.toString()),
  msgChannel: z
    .bigint()
    .transform((x) => x.toString())
    .nullish(),
  logMsg: z
    .bigint()
    .transform((x) => x.toString())
    .nullish(),
  logMod: z
    .bigint()
    .transform((x) => x.toString())
    .nullish(),
  logMember: z
    .bigint()
    .transform((x) => x.toString())
    .nullish(),
  muteRole: z
    .bigint()
    .transform((x) => x.toString())
    .nullish(),
  muteDuration: z
    .bigint()
    .transform((x) => x.toString())
    .nullish(),
  disabledChannels: z
    .bigint()
    .transform((x) => x.toString())
    .array(),
});

export const TransportGuildConfig = GuildConfigModel.extend({
  id: z.string(),
  msgChannel: z.string().nullish(),
  logMsg: z.string().nullish(),
  logMod: z.string().nullish(),
  logMember: z.string().nullish(),
  muteRole: z.string().nullish(),
  muteDuration: z.string().nullish(),
  disabledChannels: z.string().array(),
});

export type TransportGuildConfigModel = z.infer<typeof TransportGuildConfig>;

// Conversion from StringGuildConfigModel to GuildConfigModel.
export const fromTransportGuildConfigModel = GuildConfigModel.extend({
  id: z.string().transform(BigInt),
  prefix: z.string().nullish(),
  joinMsg: z.string().nullish(),
  joinMsgEnabled: z.boolean(),
  joinReact: z.string().nullish(),
  leaveMsg: z.string().nullish(),
  leaveMsgEnabled: z.boolean(),
  msgChannel: z.bigint().transform(BigInt).nullish(),
  inviteGuard: z.boolean(),
  logMsg: z.bigint().transform(BigInt).nullish(),
  logMsgEnabled: z.boolean(),
  logMod: z.bigint().transform(BigInt).nullish(),
  logModEnabled: z.boolean(),
  logMember: z.bigint().transform(BigInt).nullish(),
  logMemberEnabled: z.boolean(),
  muteRole: z.bigint().transform(BigInt).nullish(),
  muteDuration: z.bigint().transform(BigInt).nullish(),
  muteDmText: z.string().nullish(),
  muteDmEnabled: z.boolean(),
  warnDmText: z.string().nullish(),
  warnDmEnabled: z.boolean(),
  maxMention: z.number().int().nullish(),
  disabledChannels: z.bigint().transform(BigInt).array(),
});

export type StoredGuildConfigModel = z.infer<
  typeof fromTransportGuildConfigModel
>;

export function getDefaultTransportGuildConfig(
  id: string,
): TransportGuildConfigModel {
  return {
    id,
    joinMsgEnabled: true,
    leaveMsgEnabled: true,
    inviteGuard: true,
    logMsgEnabled: true,
    logModEnabled: true,
    logMemberEnabled: true,
    muteDmEnabled: true,
    warnDmEnabled: true,
    disabledChannels: [],
  };
}
