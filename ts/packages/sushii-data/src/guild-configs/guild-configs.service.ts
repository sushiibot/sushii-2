import { HttpException, HttpStatus, Injectable } from '@nestjs/common';
import { PrismaService } from '../prisma.service';
import { CreateGuildConfigDto } from './dto/create-guild-config.dto';
import { UpdateGuildConfigDto } from './dto/update-guild-config.dto';
import {
  StrictGuildConfigModel,
  StringGuildConfigModel,
} from './entities/guild-config.entity';

@Injectable()
export class GuildConfigsService {
  constructor(private prisma: PrismaService) {}

  create(createGuildConfigDto: CreateGuildConfigDto) {
    return 'This action adds a new guildConfig';
  }

  async findOne(id: string) {
    const conf = await this.prisma.guildConfig.findUnique({
      where: { id: BigInt(id) },
    });

    if (!conf) {
      throw new HttpException('Guild not found', HttpStatus.NOT_FOUND);
    }

    // Converts prisma config to a string config
    return StringGuildConfigModel.parse(conf);
  }

  async update(id: string, updateGuildConfigDto: UpdateGuildConfigDto) {
    if (updateGuildConfigDto.id.toString() !== id) {
      throw new HttpException('ID cannot be changed', HttpStatus.BAD_REQUEST);
    }

    // Converts string config to prisma config
    const updatedConfStrict =
      StrictGuildConfigModel.safeParse(updateGuildConfigDto);

    if (!updatedConfStrict.success) {
      throw new HttpException('Invalid config', HttpStatus.BAD_REQUEST);
    }

    await this.prisma.guildConfig.update({
      where: { id: updatedConfStrict.data.id },
      data: updatedConfStrict.data,
    });
  }

  remove(id: string) {
    return `This action removes a #${id} guildConfig`;
  }
}
