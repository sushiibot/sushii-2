import { createZodDto } from '@anatine/zod-nestjs';
import { StringGuildConfigModel } from '../entities/guild-config.entity';

export class GetGuildConfigResponseDto extends createZodDto(
  StringGuildConfigModel,
) {}
