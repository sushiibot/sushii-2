import { Injectable } from '@nestjs/common';
import { GetGuildConfigRequest } from 'generated/sushii/guild/config/GetGuildConfigRequest';
import { GetGuildConfigResponse } from 'generated/sushii/guild/config/GetGuildConfigResponse';

@Injectable()
export class GuildConfigService {
  get(data: GetGuildConfigRequest): GetGuildConfigResponse {
    return {
      id: '123',
    };
  }
}
