import { Test, TestingModule } from '@nestjs/testing';
import { PrismaService } from '../prisma.service';
import { GuildConfigsService } from './guild-configs.service';

describe('GuildConfigsService', () => {
  let service: GuildConfigsService;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [GuildConfigsService, PrismaService],
    }).compile();

    service = module.get<GuildConfigsService>(GuildConfigsService);
  });

  it('should be defined', () => {
    expect(service).toBeDefined();
  });
});
