// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::natives::{util, GasParameters};
use move_deps::move_vm_runtime::native_functions::NativeFunction;

// The Any module hijacks just one function, from_bytes, from the util module. This
// is a friend function which cannot be used across packages, so we have it both
// in aptos_std and aptos_framework.
pub fn make_all(gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
    let natives = [(
        "from_bytes",
        util::make_native_from_bytes(gas_params.util.from_bytes),
    )];
    crate::natives::helpers::make_module_natives(natives)
}
