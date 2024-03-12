import test from 'ava';

import {
  PluginType,
  fetchAsset,
  removePlugin,
  pluginAuthorityPair,
} from '../src';
import { assertAsset, createAsset, createUmi } from './_setup';

test('it can remove a plugin from an asset', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();

  const asset = await createAsset(umi, {
    plugins: [
      pluginAuthorityPair({ type: 'Freeze', data: { frozen: false }}),
    ],
  });

  await assertAsset(t, umi, {
    asset: asset.publicKey,
    owner: umi.identity.publicKey,
    freeze: {
      authority: {
        type: 'Owner',
      },
      frozen: false,
    },
  });

  await removePlugin(umi, {
    asset: asset.publicKey,
    pluginType: PluginType.Freeze,
  }).sendAndConfirm(umi);

  const asset2 = await fetchAsset(umi, asset.publicKey);

  t.is(asset2.freeze, undefined);
});
