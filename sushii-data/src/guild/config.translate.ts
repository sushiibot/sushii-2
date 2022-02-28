import { GuildConfig as PrismaGuildConfig, Prisma } from '@prisma/client';
import { GuildConfig as ProtoGuildConfig } from '../proto/guild/config';

export function prismaGuildConfigToProto(
  prismaConfig: PrismaGuildConfig,
): ProtoGuildConfig {
  return {
    id: prismaConfig.id.toString(),
    prefix: prismaConfig.prefix || undefined,
    joinMsg: prismaConfig.joinMsg || undefined,
    joinMsgEnabled: prismaConfig.joinMsgEnabled,
    joinReact: prismaConfig.joinReact || undefined,
    leaveMsg: prismaConfig.leaveMsg || undefined,
    leaveMsgEnabled: prismaConfig.leaveMsgEnabled,
    msgChannel: prismaConfig.msgChannel?.toString(),
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
  };
}

export function protoToPrismaUpdateGuildConfig(
  conf: ProtoGuildConfig,
): Prisma.GuildConfigUpdateInput {
  return {
    id: BigInt(conf.id),
    prefix: conf.prefix,
    joinMsg: conf.joinMsg,
    joinMsgEnabled: conf.joinMsgEnabled,
    joinReact: conf.joinReact,
    leaveMsg: conf.leaveMsg,
    leaveMsgEnabled: conf.leaveMsgEnabled,
    msgChannel: conf.msgChannel ? BigInt(conf.msgChannel) : undefined,
    inviteGuard: conf.inviteGuard,
    logMsg: conf.logMsg ? BigInt(conf.logMsg) : undefined,
    logMsgEnabled: conf.logMsgEnabled,
    logMod: conf.logMod ? BigInt(conf.logMod) : undefined,
    logModEnabled: conf.logModEnabled,
    logMember: conf.logMember ? BigInt(conf.logMember) : undefined,
    logMemberEnabled: conf.logMemberEnabled,
    muteRole: conf.muteRole ? BigInt(conf.muteRole) : undefined,
    muteDuration: conf.muteDuration ? BigInt(conf.muteDuration) : undefined,
    warnDmText: conf.warnDmText,
    warnDmEnabled: conf.warnDmEnabled,
    muteDmText: conf.muteDmText,
    muteDmEnabled: conf.muteDmEnabled,
    maxMention: conf.maxMention ? parseInt(conf.maxMention, 10) : undefined,
    disabledChannels: conf.disabledChannels.map(BigInt),
  };
}

export function protoToPrismaGuildConfig(
  conf: ProtoGuildConfig,
): PrismaGuildConfig {
  return {
    id: BigInt(conf.id),
    prefix: conf.prefix || null,
    joinMsg: conf.joinMsg || null,
    joinMsgEnabled: conf.joinMsgEnabled,
    joinReact: conf.joinReact || null,
    leaveMsg: conf.leaveMsg || null,
    leaveMsgEnabled: conf.leaveMsgEnabled,
    msgChannel: conf.msgChannel ? BigInt(conf.msgChannel) : null,
    inviteGuard: conf.inviteGuard,
    logMsg: conf.logMsg ? BigInt(conf.logMsg) : null,
    logMsgEnabled: conf.logMsgEnabled,
    logMod: conf.logMod ? BigInt(conf.logMod) : null,
    logModEnabled: conf.logModEnabled,
    logMember: conf.logMember ? BigInt(conf.logMember) : null,
    logMemberEnabled: conf.logMemberEnabled,
    muteRole: conf.muteRole ? BigInt(conf.muteRole) : null,
    muteDuration: conf.muteDuration ? BigInt(conf.muteDuration) : null,
    warnDmText: conf.warnDmText || null,
    warnDmEnabled: conf.warnDmEnabled,
    muteDmText: conf.muteDmText || null,
    muteDmEnabled: conf.muteDmEnabled,
    maxMention: conf.maxMention ? parseInt(conf.maxMention, 10) : null,
    disabledChannels: conf.disabledChannels.map(BigInt),
  };
}
