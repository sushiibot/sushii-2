import { Injectable } from '@nestjs/common';
import { RpcException } from '@nestjs/microservices';
import { status } from '@grpc/grpc-js';
import { PrismaService } from '../prisma.service';
import { GetGuildConfigResponse } from '../proto/guild/config';
import { guildConfigToResponse } from './config.translate';

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

    return guildConfigToResponse(conf);
  }
}
