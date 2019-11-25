# `midi-port`

This is a Rust driver library for UART midi port. 
Currently only input is supported.

It is platform independent as it uses [embedded-hal] APIs to access hardware.
The examples are based on the [stm32f4xx_hal] implementation of embedded-hal.


# Documentation

See [examples].

# License

This library is licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

[embedded-hal]: https://docs.rs/embedded-hal/0.2.3/embedded_hal/
[stm32f4xx_hal]: https://docs.rs/stm32f4xx-hal/0.5.0/stm32f4xx_hal/
[examples]: https://github.com/wjakobczyk/st7920/tree/master/examples
