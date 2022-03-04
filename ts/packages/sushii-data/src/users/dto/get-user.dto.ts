import { createZodDto } from '@anatine/zod-nestjs';
import { fromStoredUserModel } from '../entities/user.entity';

export class GetUserResponseDto extends createZodDto(fromStoredUserModel) {}
