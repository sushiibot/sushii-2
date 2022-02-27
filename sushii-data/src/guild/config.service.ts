import { Injectable } from '@nestjs/common';
import { RpcException } from '@nestjs/microservices';
import { status } from '@grpc/grpc-js';
import { PrismaService } from '../prisma.service';
import { Prisma } from '@prisma/client';
import {
  GetGuildConfigResponse,
  GuildConfig,
  UpdateGuildConfigResponse,
} from '../proto/guild/config';
import {
  prismaGuildConfigToProto,
  protoToPrismaGuildConfig,
} from './config.translate';
import { FieldMask } from '../../google/protobuf/field_mask';

@Injectable()
export class GuildConfigService {
  constructor(private prisma: PrismaService) {}

  async get(guildId: string): Promise<GetGuildConfigResponse> {
    if (guildId === '') {
      throw new RpcException({
        code: status.INVALID_ARGUMENT,
        message: 'ID cannot be empty',
      });
    }

    const conf = await this.prisma.guildConfig.findUnique({
      where: { id: BigInt(guildId) },
    });

    if (!conf) {
      throw new RpcException({
        code: status.NOT_FOUND,
        message: 'Guild not found',
      });
    }

    const protoConf = prismaGuildConfigToProto(conf);

    return {
      config: protoConf,
    };
  }

  async update(
    config: GuildConfig | undefined,
    fieldMask: FieldMask | undefined,
  ): Promise<UpdateGuildConfigResponse> {
    if (!config) {
      throw new RpcException({
        code: status.INVALID_ARGUMENT,
        message: 'config cannot be empty',
      });
    }

    if (!fieldMask) {
      throw new RpcException({
        code: status.INVALID_ARGUMENT,
        message: 'field mask cannot be empty',
      });
    }
    const prismaConf = protoToPrismaGuildConfig(config);

    const updateInput: Prisma.GuildConfigUpdateInput = {
      id: prismaConf.id,
      prefix: prismaConf.prefix,
      joinMsg: prismaConf.joinMsg,
      joinMsgEnabled: prismaConf.joinMsgEnabled,
      joinReact: prismaConf.joinReact,
      leaveMsg: prismaConf.leaveMsg,
      leaveMsgEnabled: prismaConf.leaveMsgEnabled,
      msgChannel: prismaConf.msgChannel,
      inviteGuard: prismaConf.inviteGuard,
      logMsg: prismaConf.logMsg,
      logMsgEnabled: prismaConf.logMsgEnabled,
      logMod: prismaConf.logMod,
      logModEnabled: prismaConf.logModEnabled,
      logMember: prismaConf.logMember,
      logMemberEnabled: prismaConf.logMemberEnabled,
      muteRole: prismaConf.muteRole,
      muteDuration: prismaConf.muteDuration,
      warnDmText: prismaConf.warnDmText,
      warnDmEnabled: prismaConf.warnDmEnabled,
      muteDmText: prismaConf.muteDmText,
      muteDmEnabled: prismaConf.muteDmEnabled,
      maxMention: prismaConf.maxMention,
      disabledChannels: prismaConf.disabledChannels,
    };

    await this.prisma.guildConfig.update({
      where: { id: BigInt(config.id) },
      data: updateInput,
    });

    return {};
  }
}
