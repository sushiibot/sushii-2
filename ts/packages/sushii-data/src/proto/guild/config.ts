/* eslint-disable */
import { GrpcMethod, GrpcStreamMethod } from '@nestjs/microservices';
import { util, configure } from 'protobufjs/minimal';
import * as Long from 'long';
import { Observable } from 'rxjs';

export const protobufPackage = 'sushii.guild.config';

/** Get */
export interface GetGuildConfigRequest {
  id: string;
}

export interface GetGuildConfigResponse {
  config: GuildConfig | undefined;
}

/** Update() */
export interface UpdateGuildConfigRequest {
  guild: GuildConfig | undefined;
}

export interface UpdateGuildConfigResponse {}

/** Create() */
export interface CreateGuildConfigRequest {
  id: string;
}

export interface CreateGuildConfigResponse {}

export interface GuildConfig {
  id: string;
  prefix?: string | undefined;
  joinMsg?: string | undefined;
  joinMsgEnabled: boolean;
  joinReact?: string | undefined;
  leaveMsg?: string | undefined;
  leaveMsgEnabled: boolean;
  msgChannel?: string | undefined;
  inviteGuard: boolean;
  logMsg?: string | undefined;
  logMsgEnabled: boolean;
  logMod?: string | undefined;
  logModEnabled: boolean;
  logMember?: string | undefined;
  logMemberEnabled: boolean;
  muteRole?: string | undefined;
  muteDuration?: string | undefined;
  warnDmText?: string | undefined;
  warnDmEnabled: boolean;
  muteDmText?: string | undefined;
  muteDmEnabled: boolean;
  maxMention?: string | undefined;
  disabledChannels: string[];
}

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

interface Rpc {
  request(
    service: string,
    method: string,
    data: Uint8Array,
  ): Promise<Uint8Array>;
}

// If you get a compile-error about 'Constructor<Long> and ... have no overlap',
// add '--ts_proto_opt=esModuleInterop=true' as a flag when calling 'protoc'.
if (util.Long !== Long) {
  util.Long = Long as any;
  configure();
}
