import { Injectable } from '@nestjs/common';
import { RpcException } from '@nestjs/microservices';
import { status } from '@grpc/grpc-js';
import { PrismaService } from 'src/prisma.service';
import { GetGuildConfigResponse } from 'src/generated/src/proto/guild/config';

@Injectable()
export class GuildConfigService {
  constructor(private prisma: PrismaService) {}

  get(guildId: string): GetGuildConfigResponse {
    if (guildId === '') {
      throw new RpcException({
        code: status.INVALID_ARGUMENT,
        message: 'ID cannot be empty',
      });
    }

    return {
      id: '123',
      prefix: undefined,
      joinMsg: undefined,
      joinMsgEnabled: false,
      joinReact: undefined,
      leaveMsg: undefined,
      leaveMsgEnabled: false,
      msgChannel: undefined,
      roleChannel: undefined,
      roleConfig: undefined,
      roleEnabled: false,
      inviteGuard: false,
      logMsg: undefined,
      logMsgEnabled: false,
      logMod: undefined,
      logModEnabled: false,
      logMember: undefined,
      logMemberEnabled: false,
      muteRole: undefined,
      muteDuration: undefined,
      warnDmText: undefined,
      warnDmEnabled: false,
      muteDmText: undefined,
      muteDmEnabled: false,
      maxMention: undefined,
      disabledChannels: [],
      data: undefined,
    };
  }
}
