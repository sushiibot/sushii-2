import { HttpException, HttpStatus, Injectable } from '@nestjs/common';
import { PrismaService } from '../prisma.service';
import { UpdateGuildConfigDto } from './dto/update-guild-config.dto';
import {
  getDefaultTransportGuildConfig,
  StoredGuildConfigModel,
  TransportGuildConfigModel,
  transportGuildConfigModel,
} from './entities/guild-config.entity';

@Injectable()
export class GuildConfigsService {
  constructor(private prisma: PrismaService) {}

  async findOne(id: string): Promise<TransportGuildConfigModel> {
    const conf = await this.prisma.guildConfig.findUnique({
      where: { id: BigInt(id) },
    });

    if (!conf) {
      return getDefaultTransportGuildConfig(id);
    }

    // Converts prisma config to a string config
    return transportGuildConfigModel.parse(conf);
  }

  async update(
    id: string,
    updateGuildConfigDto: UpdateGuildConfigDto,
  ): Promise<void> {
    if (updateGuildConfigDto.id.toString() !== id) {
      throw new HttpException('ID cannot be changed', HttpStatus.BAD_REQUEST);
    }

    // Converts string config to prisma config
    const updatedConfStrict =
      StoredGuildConfigModel.safeParse(updateGuildConfigDto);

    if (!updatedConfStrict.success) {
      throw new HttpException('Invalid config', HttpStatus.BAD_REQUEST);
    }

    await this.prisma.guildConfig.update({
      where: { id: updatedConfStrict.data.id },
      data: updatedConfStrict.data,
    });
  }

  remove(id: string): void {
    `This action removes a #${id} guildConfig`;
  }
}
