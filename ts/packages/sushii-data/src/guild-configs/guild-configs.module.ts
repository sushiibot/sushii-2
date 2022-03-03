import { Module } from '@nestjs/common';
import { GuildConfigsService } from './guild-configs.service';
import { GuildConfigsController } from './guild-configs.controller';
import { PrismaModule } from '../prisma/prisma.module';

@Module({
  imports: [PrismaModule],
  controllers: [GuildConfigsController],
  providers: [GuildConfigsService],
})
export class GuildConfigsModule {}
