// Copyright 2017 Amagicom AB.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// The main macro of this crate. Generates JSON-RPC 2.0 client structs with automatic serialization
/// and deserialization. Method calls get correct types automatically.
///
/// Optional [param_structure] annotation accepts two arguments:
///  - by_name: parameters are passed as a dictionary
///  - by_position: parameters are passed as an array
/// by_position parameter passing strategy is applied by default
#[macro_export]
macro_rules! jsonrpc_client {
    (
        $(#[$struct_attr:meta])*
        pub struct $struct_name:ident {$(
            $(#[$attr:meta])*
            $([param_structure = $param_structure:ident])?
            pub fn $method:ident(&mut $selff:ident $(, $arg_name:ident: $arg_ty:ty)*)
                -> Future<$return_ty:ty>;
        )*}
    ) => (
        $(#[$struct_attr])*
        pub struct $struct_name {
            client: $crate::ClientHandle,
        }

        impl $struct_name {
            /// Creates a new RPC client backed by the given transport implementation.
            pub fn new(client: $crate::ClientHandle) -> Self {
                $struct_name { client }
            }

            $(
                jsonrpc_client_method!{
                    $(#[$attr])*
                    $([param_structure = $param_structure])?
                    pub fn $method(&mut $selff $(, $arg_name: $arg_ty)*)
                        -> Future<$return_ty>;
                }
            )*
        }
    )
}

#[doc(hidden)]
#[macro_export]
macro_rules! jsonrpc_client_method {
    (
        $(#[$attr:meta])*
        pub fn $method:ident(&mut $selff:ident $(, $arg_name:ident: $arg_ty:ty)*)
            -> Future<$return_ty:ty>;
    ) => {
        jsonrpc_client_method! {
            $(#[$attr])*
            [param_structure = by_position]
            pub fn $method(&mut $selff $(, $arg_name : $arg_ty)*)
                -> Future<$return_ty>;
        }
    };

    (
        $(#[$attr:meta])*
        [param_structure = $param_structure:ident]
        pub fn $method:ident(&mut $selff:ident $(, $arg_name:ident: $arg_ty:ty)*)
            -> Future<$return_ty:ty>;
    ) => {
        $(#[$attr])*
        pub fn $method(&mut $selff $(, $arg_name: $arg_ty)*)
            -> impl $crate::Future<Item = $return_ty, Error = $crate::Error> + 'static
        {
            let method = String::from(stringify!($method));
            let raw_params = $crate::$param_structure!($($arg_name,)*);
            let params = raw_params.and_then(|p| $crate::serialize_parameters(&p));
            let (tx, rx) = $crate::oneshot::channel();
            let client_call = params.map(|p| $crate::OutgoingMessage::RpcCall(method, p, tx));
            $selff.client.send_client_call(client_call, rx)
        }
    };
}

/// Expands a variable list of parameters into its serializable form. Is needed to make the params
/// of a nullary method equal to `[]` instead of `()` and thus make sure it serializes to `[]`
/// instead of `null`.
#[doc(hidden)]
#[macro_export]
macro_rules! by_position {
    () => (Ok([] as [(); 0]));
    ($($arg_name:ident,)+) => (Ok(($($arg_name,)+)))
}

#[doc(hidden)]
#[macro_export]
macro_rules! by_name {
    () => (Ok(std::collections::HashMap::<&'static str, serde_json::Value>::new()));
    ($($arg_name:ident,)+) => (
        // Create a slice of (&str, Value) tuples, and collect it into HashMap
        || -> Result<std::collections::HashMap<&'static str, serde_json::Value>, $crate::Error> {
            let mut map = std::collections::HashMap::new();
            $(
                let key = stringify!($arg_name);
                let value = serde_json::to_value(&$arg_name)
                    .map_err(|_| $crate::ErrorKind::SerializeError)?;
                map.insert(key, value);
            )+
            Ok(map)
        }()
    )
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]

    jsonrpc_client! {
        pub struct TestRpcClient {
            pub fn default_param_structure(&mut self) -> Future<()>;

            #[doc(hidden)]
            pub fn default_param_structure_with_attribute(&mut self) -> Future<()>;

            #[doc(hidden)]
            [param_structure = by_name]
            pub fn by_name_with_attribute(&mut self) -> Future<()>;

            [param_structure = by_position]
            pub fn by_position_with_args(&mut self, input: &str) -> Future<String>;

            [param_structure = by_name]
            pub fn by_name_with_args(&mut self, arg0: String, arg1: u64) -> Future<String>;
        }
    }

    #[test]
    fn syntax() {
        // do nothing
    }
}