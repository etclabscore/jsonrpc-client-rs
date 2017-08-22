// Copyright 2017 Amagicom AB.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// The main macro of this crate. Generates JSON-RPC 2.0 client structs with automatic serialization
/// and deserialization. Method calls get correct types automatically.
#[macro_export]
macro_rules! jsonrpc_client {
    (
        $(#[$struct_doc:meta])*
        pub struct $struct_name:ident {$(
            $(#[$doc:meta])*
            pub fn $method:ident(&mut $selff:ident $(, $arg_name:ident: $arg_ty:ty)*)
                -> RpcRequest<$return_ty:ty>;
        )*}
    ) => (
        $(#[$struct_doc])*
        pub struct $struct_name<E, T>
            where E: ::std::error::Error + Send + 'static, T: $crate::Transport<E>
        {
            transport: T,
            id: u64,
            _error: ::std::marker::PhantomData<E>,
        }

        impl<E: ::std::error::Error + Send + 'static, T: $crate::Transport<E>> $struct_name<E, T> {
            /// Creates a new RPC client backed by the given transport implementation.
            pub fn new(transport: T) -> Self {
                $struct_name {
                    transport,
                    id: 0,
                    _error: ::std::marker::PhantomData,
                }
            }

            $(
                $(#[$doc])*
                pub fn $method(&mut $selff $(, $arg_name: $arg_ty)*)
                    -> $crate::RpcRequest<$return_ty>
                {
                    $selff.id += 1;
                    let method = stringify!($method);
                    let params = expand_params!($($arg_name,)*);
                    $crate::call_method($selff.transport.clone(), $selff.id, method, params)
                }
            )*
        }
    )
}

/// Expands a variable list of parameters into its serializable form. Is needed to make the params
/// of a nullary method `[]` instead of `()` and thus make sure it serializes to `[]` instead of
/// `null`.
#[macro_export]
macro_rules! expand_params {
    () => ([] as [(); 0]);
    ($($arg_name:ident,)+) => (($($arg_name,)+))
}
