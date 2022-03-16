import { INestApplication, Injectable, OnModuleInit } from '@nestjs/common';
import { Logger } from '@nestjs/common';
import { doesNotMatch } from 'assert';
import { createClient, RedisClientType } from 'redis';

@Injectable()
export class RedisService implements OnModuleInit {
  private readonly logger = new Logger(RedisService.name);
  public client: RedisClientType | undefined;

  async onModuleInit() {
    this.client = createClient();
    this.client.on('error', (err) =>
      this.logger.error('Redis Client Error', err),
    );

    await this.client.connect();

    this.logger.log('Connected to redis');
  }

  async onApplicationShutdown() {
    await this.client?.quit();
  }
}
