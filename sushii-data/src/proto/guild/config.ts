/* eslint-disable */
import { GrpcMethod, GrpcStreamMethod } from '@nestjs/microservices';
import { util, configure } from 'protobufjs/minimal';
import * as Long from 'long';
import { Observable } from 'rxjs';
import { FieldMask } from '../../../google/protobuf/field_mask';

export const protobufPackage = 'sushii.guild.config';

export interface GetGuildConfigRequest {
  id: string;
}

export interface GetGuildConfigResponse {
  id: string;
  prefix: string | undefined;
  joinMsg: string | undefined;
  joinMsgEnabled: boolean;
  joinReact: string | undefined;
  leaveMsg: string | undefined;
  leaveMsgEnabled: boolean;
  msgChannel: string | undefined;
  roleChannel: string | undefined;
  roleConfig?: { [key: string]: any } | undefined;
  roleEnabled: boolean;
  inviteGuard: boolean;
  logMsg: string | undefined;
  logMsgEnabled: boolean;
  logMod: string | undefined;
  logModEnabled: boolean;
  logMember: string | undefined;
  logMemberEnabled: boolean;
  muteRole: string | undefined;
  muteDuration: string | undefined;
  warnDmText: string | undefined;
  warnDmEnabled: boolean;
  muteDmText: string | undefined;
  muteDmEnabled: boolean;
  maxMention: string | undefined;
  disabledChannels: string[];
  data: { [key: string]: any } | undefined;
}

export interface UpdateGuildConfigRequest {
  guild: GetGuildConfigResponse | undefined;
  updateMask: FieldMask | undefined;
}

export interface UpdateGuildConfigResponse {}

export interface CreateGuildConfigRequest {
  id: string;
}

export interface CreateGuildConfigResponse {}

export const SUSHII_GUILD_CONFIG_PACKAGE_NAME = 'sushii.guild.config';

export interface GuildConfigServiceClient {
  get(request: GetGuildConfigRequest): Observable<GetGuildConfigResponse>;

  update(
    request: UpdateGuildConfigRequest,
  ): Observable<UpdateGuildConfigResponse>;

  create(
    request: CreateGuildConfigRequest,
  ): Observable<CreateGuildConfigResponse>;
}

export interface GuildConfigServiceController {
  get(
    request: GetGuildConfigRequest,
  ):
    | Promise<GetGuildConfigResponse>
    | Observable<GetGuildConfigResponse>
    | GetGuildConfigResponse;

  update(
    request: UpdateGuildConfigRequest,
  ):
    | Promise<UpdateGuildConfigResponse>
    | Observable<UpdateGuildConfigResponse>
    | UpdateGuildConfigResponse;

  create(
    request: CreateGuildConfigRequest,
  ):
    | Promise<CreateGuildConfigResponse>
    | Observable<CreateGuildConfigResponse>
    | CreateGuildConfigResponse;
}

export function GuildConfigServiceControllerMethods() {
  return function (constructor: Function) {
    const grpcMethods: string[] = ['get', 'update', 'create'];
    for (const method of grpcMethods) {
      const descriptor: any = Reflect.getOwnPropertyDescriptor(
        constructor.prototype,
        method,
      );
      GrpcMethod('GuildConfigService', method)(
        constructor.prototype[method],
        method,
        descriptor,
      );
    }
    const grpcStreamMethods: string[] = [];
    for (const method of grpcStreamMethods) {
      const descriptor: any = Reflect.getOwnPropertyDescriptor(
        constructor.prototype,
        method,
      );
      GrpcStreamMethod('GuildConfigService', method)(
        constructor.prototype[method],
        method,
        descriptor,
      );
    }
  };
}

export const GUILD_CONFIG_SERVICE_NAME = 'GuildConfigService';

// If you get a compile-error about 'Constructor<Long> and ... have no overlap',
// add '--ts_proto_opt=esModuleInterop=true' as a flag when calling 'protoc'.
if (util.Long !== Long) {
  util.Long = Long as any;
  configure();
}
