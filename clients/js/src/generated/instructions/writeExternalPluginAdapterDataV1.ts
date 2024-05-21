/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  bytes,
  mapSerializer,
  struct,
  u32,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';
import {
  BaseExternalPluginAdapterKey,
  BaseExternalPluginAdapterKeyArgs,
  getBaseExternalPluginAdapterKeySerializer,
} from '../types';

// Accounts.
export type WriteExternalPluginAdapterDataV1InstructionAccounts = {
  /** The address of the asset */
  asset: PublicKey | Pda;
  /** The collection to which the asset belongs */
  collection?: PublicKey | Pda;
  /** The account paying for the storage fees */
  payer?: Signer;
  /** The Data Authority of the External PluginExternalPluginAdapter */
  authority?: Signer;
  /** The system program */
  systemProgram?: PublicKey | Pda;
  /** The SPL Noop Program */
  logWrapper?: PublicKey | Pda;
};

// Data.
export type WriteExternalPluginAdapterDataV1InstructionData = {
  discriminator: number;
  key: BaseExternalPluginAdapterKey;
  data: Uint8Array;
};

export type WriteExternalPluginAdapterDataV1InstructionDataArgs = {
  key: BaseExternalPluginAdapterKeyArgs;
  data: Uint8Array;
};

export function getWriteExternalPluginAdapterDataV1InstructionDataSerializer(): Serializer<
  WriteExternalPluginAdapterDataV1InstructionDataArgs,
  WriteExternalPluginAdapterDataV1InstructionData
> {
  return mapSerializer<
    WriteExternalPluginAdapterDataV1InstructionDataArgs,
    any,
    WriteExternalPluginAdapterDataV1InstructionData
  >(
    struct<WriteExternalPluginAdapterDataV1InstructionData>(
      [
        ['discriminator', u8()],
        ['key', getBaseExternalPluginAdapterKeySerializer()],
        ['data', bytes({ size: u32() })],
      ],
      { description: 'WriteExternalPluginAdapterDataV1InstructionData' }
    ),
    (value) => ({ ...value, discriminator: 28 })
  ) as Serializer<
    WriteExternalPluginAdapterDataV1InstructionDataArgs,
    WriteExternalPluginAdapterDataV1InstructionData
  >;
}

// Args.
export type WriteExternalPluginAdapterDataV1InstructionArgs =
  WriteExternalPluginAdapterDataV1InstructionDataArgs;

// Instruction.
export function writeExternalPluginAdapterDataV1(
  context: Pick<Context, 'payer' | 'programs'>,
  input: WriteExternalPluginAdapterDataV1InstructionAccounts &
    WriteExternalPluginAdapterDataV1InstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'mplCore',
    'CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d'
  );

  // Accounts.
  const resolvedAccounts = {
    asset: {
      index: 0,
      isWritable: true as boolean,
      value: input.asset ?? null,
    },
    collection: {
      index: 1,
      isWritable: true as boolean,
      value: input.collection ?? null,
    },
    payer: {
      index: 2,
      isWritable: true as boolean,
      value: input.payer ?? null,
    },
    authority: {
      index: 3,
      isWritable: false as boolean,
      value: input.authority ?? null,
    },
    systemProgram: {
      index: 4,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
    logWrapper: {
      index: 5,
      isWritable: false as boolean,
      value: input.logWrapper ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: WriteExternalPluginAdapterDataV1InstructionArgs = {
    ...input,
  };

  // Default values.
  if (!resolvedAccounts.payer.value) {
    resolvedAccounts.payer.value = context.payer;
  }
  if (!resolvedAccounts.systemProgram.value) {
    resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
      'splSystem',
      '11111111111111111111111111111111'
    );
    resolvedAccounts.systemProgram.isWritable = false;
  }

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data =
    getWriteExternalPluginAdapterDataV1InstructionDataSerializer().serialize(
      resolvedArgs as WriteExternalPluginAdapterDataV1InstructionDataArgs
    );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}