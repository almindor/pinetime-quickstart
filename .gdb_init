target extended-remote :3333

# print demangled symbols
set print asm-demangle on

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

# Enable jlink semihosting
monitor semihosting enable
# monitor flash breakpoints 1
# telnet
# monitor semihosting IOClient 3

monitor reset

# load program
load

# start the process but immediately halt the processor
stepi
