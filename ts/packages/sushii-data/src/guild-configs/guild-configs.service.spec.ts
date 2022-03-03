import { Test, TestingModule } from '@nestjs/testing';
import { PrismaService } from '../prisma/prisma.service';
import { getDefaultTransportGuildConfig } from './entities/guild-config.entity';
import { GuildConfigsService } from './guild-configs.service';

describe('GuildConfigsService', () => {
  let service: GuildConfigsService;
  let prismaService: PrismaService;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [GuildConfigsService, PrismaService],
    }).compile();

    service = module.get<GuildConfigsService>(GuildConfigsService);
    prismaService = module.get<PrismaService>(PrismaService);
  });

  describe('get', () => {
    it('should return default config if not found', async () => {
      const defaultConf = getDefaultTransportGuildConfig('1234');

      return expect(service.findOne('1234')).resolves.toEqual(defaultConf);
    });
  });

  it('should be defined', () => {
    expect(service).toBeDefined();
  });
});
