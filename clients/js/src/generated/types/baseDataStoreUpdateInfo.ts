/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Option, OptionOrNullable } from '@metaplex-foundation/umi';
import {
  Serializer,
  option,
  struct,
} from '@metaplex-foundation/umi/serializers';
import {
  ExternalPluginAdapterSchema,
  ExternalPluginAdapterSchemaArgs,
  getExternalPluginAdapterSchemaSerializer,
} from '.';

export type BaseDataStoreUpdateInfo = {
  schema: Option<ExternalPluginAdapterSchema>;
};

export type BaseDataStoreUpdateInfoArgs = {
  schema: OptionOrNullable<ExternalPluginAdapterSchemaArgs>;
};

export function getBaseDataStoreUpdateInfoSerializer(): Serializer<
  BaseDataStoreUpdateInfoArgs,
  BaseDataStoreUpdateInfo
> {
  return struct<BaseDataStoreUpdateInfo>(
    [['schema', option(getExternalPluginAdapterSchemaSerializer())]],
    { description: 'BaseDataStoreUpdateInfo' }
  ) as Serializer<BaseDataStoreUpdateInfoArgs, BaseDataStoreUpdateInfo>;
}
