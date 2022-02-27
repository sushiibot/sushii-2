import { Test, TestingModule } from '@nestjs/testing';
import { PrismaService } from '../prisma.service';
import { GuildConfigController } from './config.controller';
import { GuildConfigService } from './config.service';

describe('GuildConfigController', () => {
  let guildConfigController: GuildConfigController;
  let guildConfigService: GuildConfigService;

  beforeEach(async () => {
    const app: TestingModule = await Test.createTestingModule({
      controllers: [GuildConfigController],
      providers: [GuildConfigService, PrismaService],
    }).compile();

    guildConfigController = app.get<GuildConfigController>(
      GuildConfigController,
    );
    guildConfigService = app.get<GuildConfigService>(GuildConfigService);
  });

  describe('get', () => {
    it('should return with id 1234', () => {
      const result = {
        id: '123',
        prefix: undefined,
        joinMsg: undefined,
        joinMsgEnabled: false,
        joinReact: undefined,
        leaveMsg: undefined,
        leaveMsgEnabled: false,
        msgChannel: undefined,
        inviteGuard: false,
        logMsg: undefined,
        logMsgEnabled: false,
        logMod: undefined,
        logModEnabled: false,
        logMember: undefined,
        logMemberEnabled: false,
        muteRole: undefined,
        muteDuration: undefined,
        warnDmText: undefined,
        warnDmEnabled: false,
        muteDmText: undefined,
        muteDmEnabled: false,
        maxMention: undefined,
        disabledChannels: [],
      };

      const spy = jest
        .spyOn(guildConfigService, 'get')
        .mockImplementation(async () => ({ config: result }));

      expect(guildConfigController.get({ id: '11' })).resolves.toStrictEqual({
        config: result,
      });

      expect(spy).toHaveBeenCalledWith('11');
    });
  });
});
