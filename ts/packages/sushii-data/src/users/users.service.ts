import { HttpException, HttpStatus, Injectable } from '@nestjs/common';
import { PrismaService } from '../prisma/prisma.service';
import { UpdateUserDto } from './dto/update-user.dto';
import {
  fromStoredUserModel,
  fromTransportUserModel,
  getDefaultTransportUserModel,
  TransportUserModel,
} from './entities/user.entity';

@Injectable()
export class UsersService {
  constructor(private prisma: PrismaService) {}

  async findOne(id: string): Promise<TransportUserModel> {
    const user = await this.prisma.user.findUnique({
      where: { id: BigInt(id) },
    });

    if (!user) {
      return getDefaultTransportUserModel(id);
    }

    return fromStoredUserModel.parse(user);
  }

  async update(id: string, updateUserDto: UpdateUserDto): Promise<void> {
    if (updateUserDto.id.toString() !== id) {
      throw new HttpException('ID cannot be changed', HttpStatus.BAD_REQUEST);
    }

    // Converts string config to prisma config
    const updatedUserStrict = fromTransportUserModel.safeParse(updateUserDto);

    if (!updatedUserStrict.success) {
      throw new HttpException('Invalid user', HttpStatus.BAD_REQUEST);
    }

    await this.prisma.user.update({
      where: { id: updatedUserStrict.data.id },
      data: updatedUserStrict.data,
    });
  }

  remove(id: string) {
    return `This action removes a #${id} user`;
  }
}
