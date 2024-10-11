# Vrooom

Simple no-std algorithms for robotics.

## Features

- `std`: Enables use of the standard library for math.
- `libm`: Enables the use of the `libm` crate for math.
- `vexide-core`: Enables the use of the `vexide-core` crate for optimized math for specifically VEX V5 Brains. This feature should not be used on any other target!

The `libm` and `vexide-core` features should not be enabled at the same time. 
If the `std` feature is enabled, `libm` and `vexide-core` will be ignored.