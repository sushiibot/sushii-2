import { Module } from '@nestjs/common';
import { GuildConfigsService } from './guild-configs.service';
import { GuildConfigsController } from './guild-configs.controller';

@Module({
  controllers: [GuildConfigsController],
  providers: [GuildConfigsService],
})
export class GuildConfigsModule {}
