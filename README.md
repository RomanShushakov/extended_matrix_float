## Extended matrix float

To support types flexibility within matrix elements values in [extended matrix](https://crates.io/crates/extended_matrix) the `extended_matrix_float` lib has been created. The lib contains only one `MyFloatTrait` which have been implemented for f32 and f64 types. `MyFloatTrait` contains only standard math methods like `my_sin`, `my_cos`, `my_sqrt` ... etc. which needed to support matrix methods.
