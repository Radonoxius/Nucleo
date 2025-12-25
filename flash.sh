#! /bin/bash

mkdir native/build 2> /dev/null
cd native/build
clang -c -O3 ../asm/startup/startup.s --target=thumbv7em-none-eabihf -mfloat-abi=hard -mcpu=cortex-m4 -mfpu=fpv4-sp-d16
llvm-ar -rcs libstartup.a startup.o
rm startup.o
cd ../..

cargo b --release

cd target/thumbv7em-none-eabihf/release

arm-none-eabi-objcopy -O binary nucleo-f4 kernel.img

st-flash write kernel.img 0x08000000

cd ../../..