### Prerequisites

Have [tdlib](https://github.com/tdlib/td) built and `libtdjson.so` installed into library search path (here [are](https://unix.stackexchange.com/questions/22926/where-do-executables-look-for-shared-objects-at-runtime) [some](http://tldp.org/HOWTO/Program-Library-HOWTO/shared-libraries.html) [links](http://man7.org/linux/man-pages/man8/ld.so.8.html)), usually `.so` file should be copied somewhere like `/usr/lib`, `/usr/lib64` or `/lib64`.

### Using and running

See `examples`

Get your `app_id` and `app_hash` at https://my.telegram.org/apps and run

    $ API_ID=... API_HASH=... cargo run --example get_me
