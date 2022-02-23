import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { GuildConfigController } from './guild/config.controller';

@Module({
  imports: [],
  controllers: [AppController, GuildConfigController],
  providers: [AppService],
})
export class AppModule {}
