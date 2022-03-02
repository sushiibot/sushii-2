import { createZodDto } from '@anatine/zod-nestjs';
import { transportGuildConfigModel } from '../entities/guild-config.entity';

export class UpdateGuildConfigDto extends createZodDto(
  transportGuildConfigModel,
) {}
