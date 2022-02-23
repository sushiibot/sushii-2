import { Controller } from '@nestjs/common';
import {
  CreateGuildConfigRequest,
  CreateGuildConfigResponse,
  GetGuildConfigRequest,
  GetGuildConfigResponse,
  GuildConfigServiceController,
  GuildConfigServiceControllerMethods,
  UpdateGuildConfigRequest,
  UpdateGuildConfigResponse,
} from 'src/generated/src/proto/guild/config';
import { GuildConfigService } from './config.service';

@Controller('sushii.guild.config')
// Generated decorator that applies all the @GrpcMethod and @GrpcStreamMethod to the right methods
@GuildConfigServiceControllerMethods()
export class GuildConfigController implements GuildConfigServiceController {
  constructor(private readonly configService: GuildConfigService) {}

  async get(request: GetGuildConfigRequest): Promise<GetGuildConfigResponse> {
    return this.configService.get(request.id);
  }

  async update(
    request: UpdateGuildConfigRequest,
  ): Promise<UpdateGuildConfigResponse> {
    return {};
  }

  async create(
    request: CreateGuildConfigRequest,
  ): Promise<CreateGuildConfigResponse> {
    return {};
  }
}
