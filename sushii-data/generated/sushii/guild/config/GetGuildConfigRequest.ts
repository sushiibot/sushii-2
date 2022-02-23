// Original file: proto/guild/config.proto

import type { Long } from '@grpc/proto-loader';

export interface GetGuildConfigRequest {
  'id'?: (number | string | Long);
}

export interface GetGuildConfigRequest__Output {
  'id': (string);
}
