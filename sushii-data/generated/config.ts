import type * as grpc from '@grpc/grpc-js';
import type { EnumTypeDefinition, MessageTypeDefinition } from '@grpc/proto-loader';

import type { GuildConfigServiceClient as _sushii_guild_config_GuildConfigServiceClient, GuildConfigServiceDefinition as _sushii_guild_config_GuildConfigServiceDefinition } from './sushii/guild/config/GuildConfigService';

type SubtypeConstructor<Constructor extends new (...args: any) => any, Subtype> = {
  new(...args: ConstructorParameters<Constructor>): Subtype;
};

export interface ProtoGrpcType {
  google: {
    protobuf: {
      BoolValue: MessageTypeDefinition
      BytesValue: MessageTypeDefinition
      DoubleValue: MessageTypeDefinition
      FieldMask: MessageTypeDefinition
      FloatValue: MessageTypeDefinition
      Int32Value: MessageTypeDefinition
      Int64Value: MessageTypeDefinition
      ListValue: MessageTypeDefinition
      NullValue: EnumTypeDefinition
      StringValue: MessageTypeDefinition
      Struct: MessageTypeDefinition
      UInt32Value: MessageTypeDefinition
      UInt64Value: MessageTypeDefinition
      Value: MessageTypeDefinition
    }
  }
  sushii: {
    guild: {
      config: {
        CreateGuildConfigRequest: MessageTypeDefinition
        CreateGuildConfigResponse: MessageTypeDefinition
        GetGuildConfigRequest: MessageTypeDefinition
        GetGuildConfigResponse: MessageTypeDefinition
        GuildConfigService: SubtypeConstructor<typeof grpc.Client, _sushii_guild_config_GuildConfigServiceClient> & { service: _sushii_guild_config_GuildConfigServiceDefinition }
        UpdateGuildConfigRequest: MessageTypeDefinition
        UpdateGuildConfigResponse: MessageTypeDefinition
      }
    }
  }
}

