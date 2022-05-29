# rust-stm32l4-pandora

- git clone https://github.com/Guozhanxin/rust-stm32l4-pandora.git
- rustup target add thumbv7em-none-eabihf
- cd uart
- cargo build --release
- arm-none-eabi-objcopy target/thumbv7em-none-eabihf/release/uart -O binary uart.bin

使用工具烧写 uart.bin 文件；打开串口调试助手，连接 ST-Link 虚拟串口；串口调试助手会显示键盘输入的内容。
