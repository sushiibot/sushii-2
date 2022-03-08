import { Module } from '@nestjs/common';
import { LoggerModule } from 'nestjs-pino';
import { UsersModule } from './users/users.module';
import { GuildConfigsModule } from './guild-configs/guild-configs.module';
import { PrismaModule } from './prisma/prisma.module';
import { RedisModule } from './redis/redis.module';

@Module({
  imports: [
    LoggerModule.forRoot(),
    UsersModule,
    GuildConfigsModule,
    PrismaModule,
    RedisModule,
  ],
  controllers: [],
})
export class AppModule {}
