import { Injectable } from '@nestjs/common';
import { RpcException } from '@nestjs/microservices';
import { status } from '@grpc/grpc-js';
import { PrismaService } from '../prisma.service';
import {
  GetGuildConfigResponse,
  GuildConfig,
  UpdateGuildConfigResponse,
} from '../proto/guild/config';
import {
  prismaGuildConfigToProto,
  protoToPrismaUpdateGuildConfig,
} from './config.translate';

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
  ): Promise<UpdateGuildConfigResponse> {
    if (!config) {
      throw new RpcException({
        code: status.INVALID_ARGUMENT,
        message: 'config cannot be empty',
      });
    }

    const updateInput = protoToPrismaUpdateGuildConfig(config);

    await this.prisma.guildConfig.update({
      where: { id: BigInt(config.id) },
      data: updateInput,
    });

    return {};
  }
}
