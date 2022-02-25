import { GuildConfig } from '@prisma/client';
import { GetGuildConfigResponse } from '../proto/guild/config';

export function guildConfigToResponse(
  prismaConfig: GuildConfig,
): GetGuildConfigResponse {
  return {
    id: prismaConfig.id.toString(),
    prefix: prismaConfig.prefix || undefined,
    joinMsg: prismaConfig.joinMsg || undefined,
    joinMsgEnabled: prismaConfig.joinMsgEnabled,
    joinReact: prismaConfig.joinReact || undefined,
    leaveMsg: prismaConfig.leaveMsg || undefined,
    leaveMsgEnabled: prismaConfig.leaveMsgEnabled,
    msgChannel: prismaConfig.msgChannel?.toString(),
    roleChannel: prismaConfig.roleChannel?.toString(),
    roleConfig: prismaConfig.roleConfig,
    roleEnabled: prismaConfig.roleEnabled,
    inviteGuard: prismaConfig.inviteGuard,
    logMsg: prismaConfig.logMsg?.toString(),
    logMsgEnabled: prismaConfig.logMsgEnabled,
    logMod: prismaConfig.logMod?.toString(),
    logModEnabled: prismaConfig.logModEnabled,
    logMember: prismaConfig.logMember?.toString(),
    logMemberEnabled: prismaConfig.logMemberEnabled,
    muteRole: prismaConfig.muteRole?.toString(),
    muteDuration: prismaConfig.muteDuration?.toString(),
    warnDmText: prismaConfig.warnDmText || undefined,
    warnDmEnabled: prismaConfig.warnDmEnabled,
    muteDmText: prismaConfig.muteDmText || undefined,
    muteDmEnabled: prismaConfig.muteDmEnabled,
    maxMention: prismaConfig.maxMention?.toString(),
    disabledChannels: prismaConfig.disabledChannels.map(BigInt.toString),
    data: prismaConfig.data,
  };
}
