import {
  Controller,
  Get,
  Post,
  Body,
  Patch,
  Param,
  Delete,
  UsePipes,
} from '@nestjs/common';
import { ApiCreatedResponse } from '@nestjs/swagger';
import { ZodValidationPipe } from '@anatine/zod-nestjs';
import { GuildConfigsService } from './guild-configs.service';
import { CreateGuildConfigDto } from './dto/create-guild-config.dto';
import { UpdateGuildConfigDto } from './dto/update-guild-config.dto';
import { GetGuildConfigResponseDto } from './dto/get-guild-config.dto';

@Controller('guild-configs')
@UsePipes(ZodValidationPipe)
export class GuildConfigsController {
  constructor(private readonly guildConfigsService: GuildConfigsService) {}

  @Post()
  create(@Body() createGuildConfigDto: CreateGuildConfigDto) {
    return this.guildConfigsService.create(createGuildConfigDto);
  }

  @Get(':id')
  @ApiCreatedResponse({
    type: GetGuildConfigResponseDto,
  })
  async findOne(@Param('id') id: string): Promise<GetGuildConfigResponseDto> {
    return this.guildConfigsService.findOne(id);
  }

  @Patch(':id')
  update(
    @Param('id') id: string,
    @Body() updateGuildConfigDto: UpdateGuildConfigDto,
  ) {
    return this.guildConfigsService.update(id, updateGuildConfigDto);
  }

  @Delete(':id')
  remove(@Param('id') id: string) {
    return this.guildConfigsService.remove(id);
  }
}
