/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Serializer, scalarEnum } from '@metaplex-foundation/umi/serializers';

export enum ExternalPluginAdapterSchema {
  Binary,
  Json,
  MsgPack,
}

export type ExternalPluginAdapterSchemaArgs = ExternalPluginAdapterSchema;

export function getExternalPluginAdapterSchemaSerializer(): Serializer<
  ExternalPluginAdapterSchemaArgs,
  ExternalPluginAdapterSchema
> {
  return scalarEnum<ExternalPluginAdapterSchema>(ExternalPluginAdapterSchema, {
    description: 'ExternalPluginAdapterSchema',
  }) as Serializer<
    ExternalPluginAdapterSchemaArgs,
    ExternalPluginAdapterSchema
  >;
}
