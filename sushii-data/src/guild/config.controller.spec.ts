import { Test, TestingModule } from '@nestjs/testing';
import { GuildConfig } from '../proto/guild/config';
import { PrismaService } from '../prisma.service';
import { GuildConfigController } from './config.controller';
import { GuildConfigService } from './config.service';
import { protoToPrismaUpdateGuildConfig } from './config.translate';

describe('GuildConfigController', () => {
  let guildConfigController: GuildConfigController;
  let guildConfigService: GuildConfigService;
  let prismaService: PrismaService;

  beforeEach(async () => {
    const app: TestingModule = await Test.createTestingModule({
      controllers: [GuildConfigController],
      providers: [GuildConfigService, PrismaService],
    }).compile();

    guildConfigController = app.get<GuildConfigController>(
      GuildConfigController,
    );
    guildConfigService = app.get<GuildConfigService>(GuildConfigService);
    prismaService = app.get<PrismaService>(PrismaService);
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

  describe('update', () => {
    it('should update non-undefined fields', async () => {
      const conf: GuildConfig = {
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

      // Don't actually do a prisma update
      const spy = jest
        .spyOn(prismaService.guildConfig, 'update')
        .mockImplementation();

      // Nothing returned
      await expect(
        guildConfigController.update({
          guild: conf,
        }),
      ).resolves.not.toThrow();

      const prismaConf = protoToPrismaUpdateGuildConfig(conf);
      expect(spy).toHaveBeenCalledWith({
        where: { id: BigInt('123') },
        data: prismaConf,
      });
    });
  });
});
