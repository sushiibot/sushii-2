import { Module } from '@nestjs/common';
import { GuildConfigController } from './guild/config.controller';
import { GuildConfigService } from './guild/config.service';
import { LoggerModule } from 'nestjs-pino';
import { PrismaService } from './prisma.service';
import { UsersModule } from './users/users.module';

@Module({
  imports: [LoggerModule.forRoot(), UsersModule],
  controllers: [GuildConfigController],
  providers: [GuildConfigService, PrismaService],
})
export class AppModule {}
