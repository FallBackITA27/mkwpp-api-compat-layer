This library is a sort of compatibility layer meant to define all the inputs/outputs with the backend.

The only manual work is:
1. Every Scope/Endpoint must have the macros in mkwpp-api-compat-layer-core/src/compatibility_layer/rust_actix/to_scope.rs
2. Every Endpoint must have an associated function in mkwpp-api-compat-layer-wasm/src/lib.rs ApiHandler
3. On Backend, each service must be declared separately (recursive macros are hard)

Missing Endpoints:
