//
// Copyright 2020-2021 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use neon::prelude::*;

/// A convenience for calling a method on an object.
///
/// Equivalent to calling `get`, downcasting to a function, and then using `call`.
pub fn call_method<'a>(
    cx: &mut impl Context<'a>,
    this: Handle<'a, impl Object>,
    method_name: &str,
    args: impl IntoIterator<Item = Handle<'a, JsValue>>,
) -> JsResult<'a, JsValue> {
    let method: Handle<JsFunction> = this.get(cx, method_name)?.downcast_or_throw(cx)?;
    method.call(cx, this, args)
}
