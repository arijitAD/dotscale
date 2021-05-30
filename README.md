# Dotscale - SCALE Codec Comparator

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/arijitAD/dotscale/Test%20and%20Deploy%20report?label=SCALE%20Codec%20Compatibility)](https://arijitad.github.io/dotscale/)

Dotscale compares a implementation of SCALE Codec with the [reference implementation](https://github.com/paritytech/parity-scale-codec) that is written in Rust and maintained by Parity Technologies. It provides (some) assurance that the implementation in a given language is safe & sound to use.

It provides a wrapper over the library using the Foreign Function Interface and compares the test result with [reference implementation](https://github.com/paritytech/parity-scale-codec) 

GitHub action is integrated to run unit tests against the Rust lib and implemented lib on each pull request and the result is published at https://arijitad.github.io/dotscale/. 

### Roadmap
- [x] Integrate with CI and publish badge and report.
- [x] Golang: [go-substrate-rpc-client](https://github.com/centrifuge/go-substrate-rpc-client/tree/master/scale)
- [x] Rust: [paritytech/parity-scale-codec](https://github.com/paritytech/parity-scale-codec)
- [ ] Python: [polkascan/py-scale-codec](https://github.com/polkascan/py-scale-codec)
- [ ] Golang: [itering/scale.go](https://github.com/itering/scale.go)
- [ ] C++: [soramitsu/scale](https://github.com/soramitsu/kagome/tree/master/core/scale)
- [ ] JavaScript: [polkadot-js/api](https://github.com/polkadot-js/api)
- [ ] AssemblyScript: [LimeChain/as-scale-codec](https://github.com/LimeChain/as-scale-codec)
- [ ] Haskell: [airalab/hs-web3](https://github.com/airalab/hs-web3)
- [ ] Java: [emeraldpay/polkaj](https://github.com/emeraldpay/polkaj)
- [ ] Ruby: [itering/scale.rb](https://github.com/itering/scale.rb)
