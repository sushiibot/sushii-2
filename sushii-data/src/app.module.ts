import { Module } from '@nestjs/common';
import { GuildConfigController } from './guild/config.controller';
import { GuildConfigService } from './guild/config.service';
import { LoggerModule } from 'nestjs-pino';
import { PrismaService } from './prisma.service';

@Module({
  imports: [LoggerModule.forRoot()],
  controllers: [GuildConfigController],
  providers: [GuildConfigService, PrismaService],
})
export class AppModule {}
