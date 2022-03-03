import { Test, TestingModule } from '@nestjs/testing';
import { GuildConfigsController } from './guild-configs.controller';
import { GuildConfigsService } from './guild-configs.service';
import {
  fromTransportGuildConfigModel,
  fromStoredGuildConfigModel,
} from '../guild-configs/entities/guild-config.entity';
import { PrismaService } from '../prisma/prisma.service';

describe('GuildConfigsController', () => {
  let controller: GuildConfigsController;
  let prismaService: PrismaService;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      controllers: [GuildConfigsController],
      providers: [GuildConfigsService, PrismaService],
    }).compile();

    controller = module.get<GuildConfigsController>(GuildConfigsController);
    prismaService = module.get<PrismaService>(PrismaService);
  });

  it('should be defined', () => {
    expect(controller).toBeDefined();
  });

  describe('update', () => {
    it('should update non-undefined fields', async () => {
      const conf = fromStoredGuildConfigModel.parse({
        id: BigInt(1234),
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
      });

      // Don't actually do a prisma update
      const spy = jest
        .spyOn(prismaService.guildConfig, 'update')
        .mockImplementation();

      // Nothing returned
      await expect(controller.update('1234', conf)).resolves.not.toThrow();

      const prismaConf = fromTransportGuildConfigModel.parse(conf);
      expect(spy).toHaveBeenCalledWith({
        where: { id: BigInt('1234') },
        data: prismaConf,
      });
    });

    it('should disallow updating id', async () => {
      const conf = fromStoredGuildConfigModel.parse({
        id: BigInt(1),
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
      });

      // Don't actually do a prisma update
      const spy = jest
        .spyOn(prismaService.guildConfig, 'update')
        .mockImplementation();

      // Nothing returned
      await expect(controller.update('1234', conf)).rejects.toThrow(
        'ID cannot be changed',
      );
    });
  });
});
