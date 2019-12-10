# `midi-port`

This is a Rust driver library for UART midi port. 

It is platform independent as it uses [embedded-hal] APIs to access hardware.
The examples are based on the [stm32f4xx_hal] implementation of embedded-hal.

Limitations and known issues:
* Only input is supported
* Only channel messages are supported, system messages are not (e.g. sysex, real-time)

# Documentation

See [examples].

On hardware side, a proper connection with use of an opto-isolator is required. [An example application.](http://nerdclub-uk.blogspot.com/2017/04/a-quick-thanks-to-jason-for-midi-in.html)

# License

This library is licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

[embedded-hal]: https://docs.rs/embedded-hal/0.2.3/embedded_hal/
[stm32f4xx_hal]: https://docs.rs/stm32f4xx-hal/0.5.0/stm32f4xx_hal/
[examples]: https://github.com/wjakobczyk/midi-port/tree/master/examples