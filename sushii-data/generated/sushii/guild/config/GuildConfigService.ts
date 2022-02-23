// Original file: proto/guild/config.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { CreateGuildConfigRequest as _sushii_guild_config_CreateGuildConfigRequest, CreateGuildConfigRequest__Output as _sushii_guild_config_CreateGuildConfigRequest__Output } from '../../../sushii/guild/config/CreateGuildConfigRequest';
import type { CreateGuildConfigResponse as _sushii_guild_config_CreateGuildConfigResponse, CreateGuildConfigResponse__Output as _sushii_guild_config_CreateGuildConfigResponse__Output } from '../../../sushii/guild/config/CreateGuildConfigResponse';
import type { GetGuildConfigRequest as _sushii_guild_config_GetGuildConfigRequest, GetGuildConfigRequest__Output as _sushii_guild_config_GetGuildConfigRequest__Output } from '../../../sushii/guild/config/GetGuildConfigRequest';
import type { GetGuildConfigResponse as _sushii_guild_config_GetGuildConfigResponse, GetGuildConfigResponse__Output as _sushii_guild_config_GetGuildConfigResponse__Output } from '../../../sushii/guild/config/GetGuildConfigResponse';
import type { UpdateGuildConfigRequest as _sushii_guild_config_UpdateGuildConfigRequest, UpdateGuildConfigRequest__Output as _sushii_guild_config_UpdateGuildConfigRequest__Output } from '../../../sushii/guild/config/UpdateGuildConfigRequest';
import type { UpdateGuildConfigResponse as _sushii_guild_config_UpdateGuildConfigResponse, UpdateGuildConfigResponse__Output as _sushii_guild_config_UpdateGuildConfigResponse__Output } from '../../../sushii/guild/config/UpdateGuildConfigResponse';

export interface GuildConfigServiceClient extends grpc.Client {
  Create(argument: _sushii_guild_config_CreateGuildConfigRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_sushii_guild_config_CreateGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  Create(argument: _sushii_guild_config_CreateGuildConfigRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_sushii_guild_config_CreateGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  Create(argument: _sushii_guild_config_CreateGuildConfigRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_sushii_guild_config_CreateGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  Create(argument: _sushii_guild_config_CreateGuildConfigRequest, callback: grpc.requestCallback<_sushii_guild_config_CreateGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  create(argument: _sushii_guild_config_CreateGuildConfigRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_sushii_guild_config_CreateGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  create(argument: _sushii_guild_config_CreateGuildConfigRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_sushii_guild_config_CreateGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  create(argument: _sushii_guild_config_CreateGuildConfigRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_sushii_guild_config_CreateGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  create(argument: _sushii_guild_config_CreateGuildConfigRequest, callback: grpc.requestCallback<_sushii_guild_config_CreateGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  
  Get(argument: _sushii_guild_config_GetGuildConfigRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_sushii_guild_config_GetGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  Get(argument: _sushii_guild_config_GetGuildConfigRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_sushii_guild_config_GetGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  Get(argument: _sushii_guild_config_GetGuildConfigRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_sushii_guild_config_GetGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  Get(argument: _sushii_guild_config_GetGuildConfigRequest, callback: grpc.requestCallback<_sushii_guild_config_GetGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  get(argument: _sushii_guild_config_GetGuildConfigRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_sushii_guild_config_GetGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  get(argument: _sushii_guild_config_GetGuildConfigRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_sushii_guild_config_GetGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  get(argument: _sushii_guild_config_GetGuildConfigRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_sushii_guild_config_GetGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  get(argument: _sushii_guild_config_GetGuildConfigRequest, callback: grpc.requestCallback<_sushii_guild_config_GetGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  
  Update(argument: _sushii_guild_config_UpdateGuildConfigRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_sushii_guild_config_UpdateGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  Update(argument: _sushii_guild_config_UpdateGuildConfigRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_sushii_guild_config_UpdateGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  Update(argument: _sushii_guild_config_UpdateGuildConfigRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_sushii_guild_config_UpdateGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  Update(argument: _sushii_guild_config_UpdateGuildConfigRequest, callback: grpc.requestCallback<_sushii_guild_config_UpdateGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  update(argument: _sushii_guild_config_UpdateGuildConfigRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_sushii_guild_config_UpdateGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  update(argument: _sushii_guild_config_UpdateGuildConfigRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_sushii_guild_config_UpdateGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  update(argument: _sushii_guild_config_UpdateGuildConfigRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_sushii_guild_config_UpdateGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  update(argument: _sushii_guild_config_UpdateGuildConfigRequest, callback: grpc.requestCallback<_sushii_guild_config_UpdateGuildConfigResponse__Output>): grpc.ClientUnaryCall;
  
}

export interface GuildConfigServiceHandlers extends grpc.UntypedServiceImplementation {
  Create: grpc.handleUnaryCall<_sushii_guild_config_CreateGuildConfigRequest__Output, _sushii_guild_config_CreateGuildConfigResponse>;
  
  Get: grpc.handleUnaryCall<_sushii_guild_config_GetGuildConfigRequest__Output, _sushii_guild_config_GetGuildConfigResponse>;
  
  Update: grpc.handleUnaryCall<_sushii_guild_config_UpdateGuildConfigRequest__Output, _sushii_guild_config_UpdateGuildConfigResponse>;
  
}

export interface GuildConfigServiceDefinition extends grpc.ServiceDefinition {
  Create: MethodDefinition<_sushii_guild_config_CreateGuildConfigRequest, _sushii_guild_config_CreateGuildConfigResponse, _sushii_guild_config_CreateGuildConfigRequest__Output, _sushii_guild_config_CreateGuildConfigResponse__Output>
  Get: MethodDefinition<_sushii_guild_config_GetGuildConfigRequest, _sushii_guild_config_GetGuildConfigResponse, _sushii_guild_config_GetGuildConfigRequest__Output, _sushii_guild_config_GetGuildConfigResponse__Output>
  Update: MethodDefinition<_sushii_guild_config_UpdateGuildConfigRequest, _sushii_guild_config_UpdateGuildConfigResponse, _sushii_guild_config_UpdateGuildConfigRequest__Output, _sushii_guild_config_UpdateGuildConfigResponse__Output>
}
