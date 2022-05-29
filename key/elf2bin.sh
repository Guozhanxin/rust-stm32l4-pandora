#!/bin/bash

project_path=$(cd `dirname $0`; pwd)
project_name="${project_path##*/}"
/opt/gcc-arm-none-eabi-10-2020-q4-major/bin/arm-none-eabi-objcopy target/thumbv7em-none-eabihf/release/$project_name -O binary $project_name.bin
echo 'make' $project_name'.bin OK!'