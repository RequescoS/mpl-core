use std::collections::BTreeMap;

use borsh::{BorshDeserialize, BorshSerialize};
use mpl_utils::assert_signer;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_memory::sol_memcpy,
};

use crate::{
    error::MplCoreError,
    instruction::accounts::{UpdateAccounts, UpdateCollectionAccounts},
    plugins::{
        validate_plugin_checks, CheckResult, Plugin, PluginType, RegistryRecord, ValidationResult,
    },
    state::{Asset, Collection, DataBlob, Key, SolanaAccount, UpdateAuthority},
    utils::{fetch_core_data, resize_or_reallocate_account},
};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub(crate) struct UpdateArgs {
    pub new_name: Option<String>,
    pub new_uri: Option<String>,
}

pub(crate) fn update<'a>(accounts: &'a [AccountInfo<'a>], args: UpdateArgs) -> ProgramResult {
    // Accounts.
    let ctx = UpdateAccounts::context(accounts)?;

    // Guards.
    assert_signer(ctx.accounts.authority)?;
    let payer = match ctx.accounts.payer {
        Some(payer) => {
            assert_signer(payer)?;
            payer
        }
        None => ctx.accounts.authority,
    };

    let (mut asset, plugin_header, plugin_registry) = fetch_core_data::<Asset>(ctx.accounts.asset)?;
    let asset_size = asset.get_size() as isize;

    let mut checks: BTreeMap<PluginType, (Key, CheckResult, RegistryRecord)> = BTreeMap::new();

    // The asset approval overrides the collection approval.
    let asset_approval = Asset::check_update();
    let core_check = match asset_approval {
        CheckResult::None => (Key::Collection, Collection::check_update()),
        _ => (Key::Asset, asset_approval),
    };

    // Check the collection plugins first.
    if let Some(collection_info) = ctx.accounts.collection {
        fetch_core_data::<Collection>(collection_info).map(|(_, _, registry)| {
            registry.map(|r| {
                r.check_transfer(Key::Collection, &mut checks);
                r
            })
        })?;
    }

    // Next check the asset plugins. Plugins on the asset override the collection plugins,
    // so we don't need to validate the collection plugins if the asset has a plugin.
    if let Some(registry) = plugin_registry.as_ref() {
        registry.check_update(Key::Asset, &mut checks);
    }

    solana_program::msg!("checks: {:#?}", checks);

    // Do the core validation.
    let mut approved = matches!(
        core_check,
        (
            Key::Asset | Key::Collection,
            CheckResult::CanApprove | CheckResult::CanReject
        )
    ) && {
        (match core_check.0 {
            Key::Collection => Collection::load(
                ctx.accounts
                    .collection
                    .ok_or(MplCoreError::InvalidCollection)?,
                0,
            )?
            .validate_update(ctx.accounts.authority)?,
            Key::Asset => {
                Asset::load(ctx.accounts.asset, 0)?.validate_update(ctx.accounts.authority)?
            }
            _ => return Err(MplCoreError::IncorrectAccount.into()),
        }) == ValidationResult::Approved
    };

    approved = validate_plugin_checks(
        Key::Collection,
        &checks,
        ctx.accounts.authority,
        ctx.accounts.asset,
        ctx.accounts.collection,
        Box::new(Plugin::validate_update),
    )? || approved;

    approved = validate_plugin_checks(
        Key::Asset,
        &checks,
        ctx.accounts.authority,
        ctx.accounts.asset,
        ctx.accounts.collection,
        Box::new(Plugin::validate_update),
    )? || approved;

    if !approved {
        return Err(MplCoreError::InvalidAuthority.into());
    }

    let mut dirty = false;
    if let Some(new_update_authority) = ctx.accounts.new_update_authority {
        asset.update_authority = UpdateAuthority::Address(*new_update_authority.key);
        dirty = true;
    }
    if let Some(new_name) = &args.new_name {
        asset.name = new_name.clone();
        dirty = true;
    }
    if let Some(new_uri) = &args.new_uri {
        asset.uri = new_uri.clone();
        dirty = true;
    }
    if dirty {
        if let (Some(mut plugin_header), Some(mut plugin_registry)) =
            (plugin_header, plugin_registry)
        {
            let new_asset_size = asset.get_size() as isize;
            let size_diff = new_asset_size
                .checked_sub(asset_size)
                .ok_or(MplCoreError::NumericalOverflow)?;
            let new_size = (ctx.accounts.asset.data_len() as isize)
                .checked_add(size_diff)
                .ok_or(MplCoreError::NumericalOverflow)?;
            let new_registry_offset = (plugin_header.plugin_registry_offset as isize)
                .checked_add(size_diff)
                .ok_or(MplCoreError::NumericalOverflow)?;
            let registry_offset = plugin_header.plugin_registry_offset;
            plugin_header.plugin_registry_offset = new_registry_offset as usize;

            let plugin_offset = asset_size
                .checked_add(size_diff)
                .ok_or(MplCoreError::NumericalOverflow)?;
            let new_plugin_offset = new_asset_size
                .checked_add(size_diff)
                .ok_or(MplCoreError::NumericalOverflow)?;

            // //TODO: This is memory intensive, we should use memmove instead probably.
            let src = ctx.accounts.asset.data.borrow()[(plugin_offset as usize)..registry_offset]
                .to_vec();

            resize_or_reallocate_account(
                ctx.accounts.asset,
                payer,
                ctx.accounts.system_program,
                new_size as usize,
            )?;

            sol_memcpy(
                &mut ctx.accounts.asset.data.borrow_mut()[(new_plugin_offset as usize)..],
                &src,
                src.len(),
            );

            plugin_header.save(ctx.accounts.asset, new_asset_size as usize)?;
            plugin_registry.registry = plugin_registry
                .registry
                .iter_mut()
                .map(|record| {
                    let new_offset = (record.offset as isize)
                        .checked_add(size_diff)
                        .ok_or(MplCoreError::NumericalOverflow)?;
                    Ok(RegistryRecord {
                        plugin_type: record.plugin_type,
                        offset: new_offset as usize,
                        authorities: record.authorities.clone(),
                    })
                })
                .collect::<Result<Vec<_>, MplCoreError>>()?;
            plugin_registry.save(ctx.accounts.asset, new_registry_offset as usize)?;
        } else {
            resize_or_reallocate_account(
                ctx.accounts.asset,
                payer,
                ctx.accounts.system_program,
                asset.get_size(),
            )?;
        }

        asset.save(ctx.accounts.asset, 0)?;
    }

    Ok(())
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub(crate) struct UpdateCollectionArgs {
    pub new_name: Option<String>,
    pub new_uri: Option<String>,
}

pub(crate) fn update_collection<'a>(
    accounts: &'a [AccountInfo<'a>],
    args: UpdateCollectionArgs,
) -> ProgramResult {
    // Accounts.
    let ctx = UpdateCollectionAccounts::context(accounts)?;

    // Guards.
    assert_signer(ctx.accounts.authority)?;
    let payer = match ctx.accounts.payer {
        Some(payer) => {
            assert_signer(payer)?;
            payer
        }
        None => ctx.accounts.authority,
    };

    let (mut asset, plugin_header, plugin_registry) =
        fetch_core_data::<Collection>(ctx.accounts.collection)?;
    let asset_size = asset.get_size() as isize;

    let mut approved = false;
    match Asset::check_update() {
        CheckResult::CanApprove => {
            if asset.validate_update(ctx.accounts.authority)? == ValidationResult::Approved {
                approved = true;
            }
        }
        CheckResult::CanReject => return Err(MplCoreError::InvalidAuthority.into()),
        CheckResult::None => (),
    };

    if let Some(plugin_registry) = plugin_registry.clone() {
        for record in plugin_registry.registry {
            if matches!(
                record.plugin_type.check_transfer(),
                CheckResult::CanApprove | CheckResult::CanReject
            ) {
                let result = Plugin::validate_update(
                    &Plugin::load(ctx.accounts.collection, record.offset)?,
                    ctx.accounts.authority,
                    &record.authorities,
                )?;
                if result == ValidationResult::Rejected {
                    return Err(MplCoreError::InvalidAuthority.into());
                } else if result == ValidationResult::Approved {
                    approved = true;
                }
            }
        }
    };

    if !approved {
        return Err(MplCoreError::InvalidAuthority.into());
    }

    let mut dirty = false;
    if let Some(new_update_authority) = ctx.accounts.new_update_authority {
        asset.update_authority = *new_update_authority.key;
        dirty = true;
    }
    if let Some(new_name) = &args.new_name {
        asset.name = new_name.clone();
        dirty = true;
    }
    if let Some(new_uri) = &args.new_uri {
        asset.uri = new_uri.clone();
        dirty = true;
    }
    if dirty {
        if let (Some(mut plugin_header), Some(mut plugin_registry)) =
            (plugin_header, plugin_registry)
        {
            let new_asset_size = asset.get_size() as isize;
            let size_diff = new_asset_size
                .checked_sub(asset_size)
                .ok_or(MplCoreError::NumericalOverflow)?;
            let new_size = (ctx.accounts.collection.data_len() as isize)
                .checked_add(size_diff)
                .ok_or(MplCoreError::NumericalOverflow)?;
            let new_registry_offset = (plugin_header.plugin_registry_offset as isize)
                .checked_add(size_diff)
                .ok_or(MplCoreError::NumericalOverflow)?;
            let registry_offset = plugin_header.plugin_registry_offset;
            plugin_header.plugin_registry_offset = new_registry_offset as usize;

            let plugin_offset = asset_size
                .checked_add(size_diff)
                .ok_or(MplCoreError::NumericalOverflow)?;
            let new_plugin_offset = new_asset_size
                .checked_add(size_diff)
                .ok_or(MplCoreError::NumericalOverflow)?;

            // //TODO: This is memory intensive, we should use memmove instead probably.
            let src = ctx.accounts.collection.data.borrow()
                [(plugin_offset as usize)..registry_offset]
                .to_vec();

            resize_or_reallocate_account(
                ctx.accounts.collection,
                payer,
                ctx.accounts.system_program,
                new_size as usize,
            )?;

            sol_memcpy(
                &mut ctx.accounts.collection.data.borrow_mut()[(new_plugin_offset as usize)..],
                &src,
                src.len(),
            );

            plugin_header.save(ctx.accounts.collection, new_asset_size as usize)?;
            plugin_registry.registry = plugin_registry
                .registry
                .iter_mut()
                .map(|record| {
                    let new_offset = (record.offset as isize)
                        .checked_add(size_diff)
                        .ok_or(MplCoreError::NumericalOverflow)?;
                    Ok(RegistryRecord {
                        plugin_type: record.plugin_type,
                        offset: new_offset as usize,
                        authorities: record.authorities.clone(),
                    })
                })
                .collect::<Result<Vec<_>, MplCoreError>>()?;
            plugin_registry.save(ctx.accounts.collection, new_registry_offset as usize)?;
        } else {
            resize_or_reallocate_account(
                ctx.accounts.collection,
                payer,
                ctx.accounts.system_program,
                asset.get_size(),
            )?;
        }

        asset.save(ctx.accounts.collection, 0)?;
    }

    Ok(())
}
