import { createZodDto } from '@anatine/zod-nestjs';
import { fromStoredGuildConfigModel } from '../entities/guild-config.entity';

export class UpdateGuildConfigDto extends createZodDto(
  fromStoredGuildConfigModel,
) {}
