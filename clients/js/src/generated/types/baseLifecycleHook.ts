/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Option, OptionOrNullable, PublicKey } from '@metaplex-foundation/umi';
import {
  Serializer,
  array,
  option,
  publicKey as publicKeySerializer,
  struct,
} from '@metaplex-foundation/umi/serializers';
import {
  BaseExtraAccount,
  BaseExtraAccountArgs,
  BasePluginAuthority,
  BasePluginAuthorityArgs,
  ExternalPluginAdapterSchema,
  ExternalPluginAdapterSchemaArgs,
  getBaseExtraAccountSerializer,
  getBasePluginAuthoritySerializer,
  getExternalPluginAdapterSchemaSerializer,
} from '.';

export type BaseLifecycleHook = {
  hookedProgram: PublicKey;
  extraAccounts: Option<Array<BaseExtraAccount>>;
  dataAuthority: Option<BasePluginAuthority>;
  schema: ExternalPluginAdapterSchema;
};

export type BaseLifecycleHookArgs = {
  hookedProgram: PublicKey;
  extraAccounts: OptionOrNullable<Array<BaseExtraAccountArgs>>;
  dataAuthority: OptionOrNullable<BasePluginAuthorityArgs>;
  schema: ExternalPluginAdapterSchemaArgs;
};

export function getBaseLifecycleHookSerializer(): Serializer<
  BaseLifecycleHookArgs,
  BaseLifecycleHook
> {
  return struct<BaseLifecycleHook>(
    [
      ['hookedProgram', publicKeySerializer()],
      ['extraAccounts', option(array(getBaseExtraAccountSerializer()))],
      ['dataAuthority', option(getBasePluginAuthoritySerializer())],
      ['schema', getExternalPluginAdapterSchemaSerializer()],
    ],
    { description: 'BaseLifecycleHook' }
  ) as Serializer<BaseLifecycleHookArgs, BaseLifecycleHook>;
}
