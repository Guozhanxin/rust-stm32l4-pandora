# rust-stm32l4-pandora

- git clone https://github.com/Guozhanxin/rust-stm32l4-pandora.git
- rustup target add thumbv7em-none-eabihf

## blinky
- cd blinky
- cargo build --release
- arm-none-eabi-objcopy target/thumbv7em-none-eabihf/release/blinky -O binary blinky.bin

将 blinky.bin 拖入 ST-Link 虚拟 U 盘完成烧录；LED_R 周期性闪烁。

## uart
- cd uart
- cargo build --release
- arm-none-eabi-objcopy target/thumbv7em-none-eabihf/release/uart -O binary uart.bin

将 uart.bin 拖入 ST-Link 虚拟 U 盘完成烧录；打开串口调试助手，连接 ST-Link 虚拟串口；串口调试助手会显示键盘输入的内容。
