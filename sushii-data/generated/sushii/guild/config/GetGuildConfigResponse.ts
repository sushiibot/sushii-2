// Original file: proto/guild/config.proto

import type { StringValue as _google_protobuf_StringValue, StringValue__Output as _google_protobuf_StringValue__Output } from '../../../google/protobuf/StringValue';
import type { UInt64Value as _google_protobuf_UInt64Value, UInt64Value__Output as _google_protobuf_UInt64Value__Output } from '../../../google/protobuf/UInt64Value';
import type { Struct as _google_protobuf_Struct, Struct__Output as _google_protobuf_Struct__Output } from '../../../google/protobuf/Struct';
import type { Long } from '@grpc/proto-loader';

export interface GetGuildConfigResponse {
  'id'?: (number | string | Long);
  'prefix'?: (_google_protobuf_StringValue | null);
  'joinMsg'?: (_google_protobuf_StringValue | null);
  'joinMsgEnabled'?: (boolean);
  'joinReact'?: (_google_protobuf_StringValue | null);
  'leaveMsg'?: (_google_protobuf_StringValue | null);
  'leaveMsgEnabled'?: (boolean);
  'msgChannel'?: (_google_protobuf_UInt64Value | null);
  'roleChannel'?: (_google_protobuf_UInt64Value | null);
  'roleConfig'?: (_google_protobuf_Struct | null);
  'roleEnabled'?: (boolean);
  'inviteGuard'?: (boolean);
  'logMsg'?: (_google_protobuf_UInt64Value | null);
  'logMsgEnabled'?: (boolean);
  'logMod'?: (_google_protobuf_UInt64Value | null);
  'logModEnabled'?: (boolean);
  'logMember'?: (_google_protobuf_UInt64Value | null);
  'logMemberEnabled'?: (boolean);
  'muteRole'?: (_google_protobuf_UInt64Value | null);
  'muteDuration'?: (_google_protobuf_UInt64Value | null);
  'warnDmText'?: (_google_protobuf_StringValue | null);
  'warnDmEnabled'?: (boolean);
  'muteDmText'?: (_google_protobuf_StringValue | null);
  'muteDmEnabled'?: (boolean);
  'maxMention'?: (_google_protobuf_UInt64Value | null);
  'disabledChannels'?: (number | string | Long)[];
  'data'?: (_google_protobuf_Struct | null);
  '_roleConfig'?: "roleConfig";
}

export interface GetGuildConfigResponse__Output {
  'id': (string);
  'prefix': (_google_protobuf_StringValue__Output | null);
  'joinMsg': (_google_protobuf_StringValue__Output | null);
  'joinMsgEnabled': (boolean);
  'joinReact': (_google_protobuf_StringValue__Output | null);
  'leaveMsg': (_google_protobuf_StringValue__Output | null);
  'leaveMsgEnabled': (boolean);
  'msgChannel': (_google_protobuf_UInt64Value__Output | null);
  'roleChannel': (_google_protobuf_UInt64Value__Output | null);
  'roleConfig'?: (_google_protobuf_Struct__Output | null);
  'roleEnabled': (boolean);
  'inviteGuard': (boolean);
  'logMsg': (_google_protobuf_UInt64Value__Output | null);
  'logMsgEnabled': (boolean);
  'logMod': (_google_protobuf_UInt64Value__Output | null);
  'logModEnabled': (boolean);
  'logMember': (_google_protobuf_UInt64Value__Output | null);
  'logMemberEnabled': (boolean);
  'muteRole': (_google_protobuf_UInt64Value__Output | null);
  'muteDuration': (_google_protobuf_UInt64Value__Output | null);
  'warnDmText': (_google_protobuf_StringValue__Output | null);
  'warnDmEnabled': (boolean);
  'muteDmText': (_google_protobuf_StringValue__Output | null);
  'muteDmEnabled': (boolean);
  'maxMention': (_google_protobuf_UInt64Value__Output | null);
  'disabledChannels': (string)[];
  'data': (_google_protobuf_Struct__Output | null);
  '_roleConfig': "roleConfig";
}
