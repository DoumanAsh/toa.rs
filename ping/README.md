# toa-ping

Kawaii ping tool

## Usage

```
Kawaii ping tool.

USAGE:
    toa-ping.exe [FLAGS] [OPTIONS] <destination>

FLAGS:
        --4          Enforce IPv4 version.
        --6          Enforce IPv6 version.
    -f, --forever    Keeps going forever.
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --interval <interval>    Time interval between pings in milliseconds. [default: 500]
    -n, --number <number>        Number of ping packets to send. [default: 4]
    -p, --protocol <protocol>    Specifies protocol to use. Available: tcp [default: tcp]
    -w, --timeout <timeout>      Time to wait for each response in milliseconds. [default: 1000]

ARGS:
    <destination>    Destination to ping.

Destination format: <host>[:<port>]
```
