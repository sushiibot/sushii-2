// Original file: proto/guild/config.proto

import type { Long } from '@grpc/proto-loader';

export interface CreateGuildConfigRequest {
  'id'?: (number | string | Long);
}

export interface CreateGuildConfigRequest__Output {
  'id': (string);
}
