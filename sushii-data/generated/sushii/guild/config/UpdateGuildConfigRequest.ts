// Original file: proto/guild/config.proto

import type { GetGuildConfigResponse as _sushii_guild_config_GetGuildConfigResponse, GetGuildConfigResponse__Output as _sushii_guild_config_GetGuildConfigResponse__Output } from '../../../sushii/guild/config/GetGuildConfigResponse';
import type { FieldMask as _google_protobuf_FieldMask, FieldMask__Output as _google_protobuf_FieldMask__Output } from '../../../google/protobuf/FieldMask';

export interface UpdateGuildConfigRequest {
  'guild'?: (_sushii_guild_config_GetGuildConfigResponse | null);
  'updateMask'?: (_google_protobuf_FieldMask | null);
}

export interface UpdateGuildConfigRequest__Output {
  'guild': (_sushii_guild_config_GetGuildConfigResponse__Output | null);
  'updateMask': (_google_protobuf_FieldMask__Output | null);
}
