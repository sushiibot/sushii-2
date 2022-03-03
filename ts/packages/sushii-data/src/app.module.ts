import { Module } from '@nestjs/common';
import { LoggerModule } from 'nestjs-pino';
import { PrismaService } from './prisma/prisma.service';
import { UsersModule } from './users/users.module';
import { GuildConfigsModule } from './guild-configs/guild-configs.module';
import { PrismaModule } from './prisma/prisma.module';

@Module({
  imports: [
    LoggerModule.forRoot(),
    UsersModule,
    GuildConfigsModule,
    PrismaModule,
  ],
  controllers: [],
})
export class AppModule {}
