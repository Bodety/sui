// Copyright (c) Mysten Labs
// SPDX-License-Identifier: Apache-2.0

use move_core_types::{
    account_address::AccountAddress, ident_str, identifier::IdentStr, language_storage::StructTag,
};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::{
    base_types::ObjectID,
    coin::Coin,
    error::{FastPayError, FastPayResult},
    id::ID,
    object::{Data, Object},
};

/// 0x01B3B1DD18A3B775FE0E0D4B873C0AA0
pub const GAS_ADDRESS: AccountAddress = AccountAddress::new([
    0x01, 0xB3, 0xB1, 0xDD, 0x18, 0xA3, 0xB7, 0x75, 0xFE, 0x0E, 0x0D, 0x4B, 0x87, 0x3C, 0x0A, 0xA0,
]);
pub const GAS_MODULE_NAME: &IdentStr = ident_str!("GAS");
pub const GAS_STRUCT_NAME: &IdentStr = GAS_MODULE_NAME;

/// Rust version of the Move FastX::Coin::Coin<FastX::GAS::GAS> type
#[derive(Debug, Serialize, Deserialize)]
pub struct GasCoin(Coin);

impl GasCoin {
    pub fn new(id: ObjectID, value: u64) -> Self {
        Self(Coin::new(ID::new(id), value))
    }

    pub fn value(&self) -> u64 {
        self.0.value()
    }

    pub fn type_() -> StructTag {
        Coin::type_(StructTag {
            address: GAS_ADDRESS,
            name: GAS_STRUCT_NAME.to_owned(),
            module: GAS_MODULE_NAME.to_owned(),
            type_params: Vec::new(),
        })
    }

    pub fn id(&self) -> &ObjectID {
        self.0.id()
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }
}

impl TryFrom<&Object> for GasCoin {
    type Error = FastPayError;

    fn try_from(value: &Object) -> FastPayResult<GasCoin> {
        match (value.type_(), &value.data) {
            (Some(t), Data::Move(obj)) => {
                if t != &GasCoin::type_() {
                    return Err(FastPayError::TypeError {
                        error: format!("Gas object type is not a gas coin: {}", t),
                    });
                }
                let gas_coin: GasCoin =
                    bcs::from_bytes(&obj.contents).map_err(|err| FastPayError::TypeError {
                        error: format!("Unable to deserialize gas object: {:?}", err),
                    })?;
                Ok(gas_coin)
            }
            _ => Err(FastPayError::TypeError {
                error: format!("Gas object type is not a gas coin: {:?}", value),
            }),
        }
    }
}