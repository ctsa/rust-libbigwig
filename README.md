# rust-libBigWig

This library provides a very limited rust API for [libBigWig](https://github.com/dpryan79/libBigWig). It was developed to provide bigwig output for [HiFiCNV](https://github.com/PacificBiosciences/HiFiCNV) and [sawfish](https://github.com/PacificBiosciences/sawfish), so provides only minimum coverage for the functions these tools need. It is unsupported.

For any rust user needing similar capabilities today, a recommended solution is [Bigtools](https://github.com/jackh726/bigtools), a rust-native library for bigwig file handling released after this wrapper was created.

## Binding details

The library is currently using code from libBigWig version 0.4.6. Bindings are generated using bindgen 0.59.2 with cmdline:

```
bindgen src/libBigWig/include/bigWig.h -o src/bindings.rs --whitelist-function '^bw.*' --whitelist-var '^bw.*' -- -DNOCURL
```
