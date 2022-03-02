import { createZodDto } from '@anatine/zod-nestjs';
import { transportGuildConfigModel } from '../entities/guild-config.entity';

export class GetGuildConfigResponseDto extends createZodDto(
  transportGuildConfigModel,
) {}
