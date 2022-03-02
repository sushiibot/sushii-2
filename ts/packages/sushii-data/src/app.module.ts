import { Module } from '@nestjs/common';
import { LoggerModule } from 'nestjs-pino';
import { PrismaService } from './prisma.service';
import { UsersModule } from './users/users.module';
import { GuildConfigsModule } from './guild-configs/guild-configs.module';

@Module({
  imports: [LoggerModule.forRoot(), UsersModule, GuildConfigsModule],
  controllers: [],
  providers: [PrismaService],
})
export class AppModule {}
