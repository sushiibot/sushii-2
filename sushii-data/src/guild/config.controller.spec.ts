import { Test, TestingModule } from '@nestjs/testing';
import { GuildConfigController } from './config.controller';
import { GuildConfigService } from './config.service';

describe('GuildConfigController', () => {
  let guildConfigController: GuildConfigController;

  beforeEach(async () => {
    const app: TestingModule = await Test.createTestingModule({
      controllers: [GuildConfigController],
      providers: [GuildConfigService],
    }).compile();

    guildConfigController = app.get<GuildConfigController>(
      GuildConfigController,
    );
  });

  describe('get', () => {
    it('should return with id 1234', () => {
      expect(guildConfigController.get({ id: '11' })).toStrictEqual({
        id: '123',
      });
    });
  });
});
