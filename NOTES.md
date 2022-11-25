# Notes

## gdb Rust u16 Bug
weird pwndbg bug (https://github.com/pwndbg/pwndbg/issues/855) .. just run following line at the beginning:
pwndbg> set language c

--> Most important types: Options Enum and Result Enum

## Result <T, E>
Rust does not support exceptions. instead we have the Result type. In rust we distinguish between recoverable and unrecoverable errors. Funktions returing a Result are implementing recoverable errors.

