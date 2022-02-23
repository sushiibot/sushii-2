import { Metadata, ServerUnaryCall } from '@grpc/grpc-js';
import { Controller } from '@nestjs/common';
import { GrpcMethod } from '@nestjs/microservices';
import { GetGuildConfigRequest } from 'generated/sushii/guild/config/GetGuildConfigRequest';
import { GetGuildConfigResponse } from 'generated/sushii/guild/config/GetGuildConfigResponse';
import { GuildConfigService } from './config.service';

@Controller()
export class GuildConfigController {
  constructor(private readonly configService: GuildConfigService) {}

  @GrpcMethod('GuildConfigService', 'Get')
  get(
    data: GetGuildConfigRequest,
    metadata: Metadata,
    call: ServerUnaryCall<GetGuildConfigRequest, GetGuildConfigResponse>,
  ): GetGuildConfigResponse {
    return this.configService.get(data);
  }
}
