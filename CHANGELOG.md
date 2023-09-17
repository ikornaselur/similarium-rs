# Changelog

## [0.4.1](https://github.com/ikornaselur/similarium-rs/compare/v0.4.0...v0.4.1) (2023-09-17)


### Bug Fixes

* use vendored openssl for reqwest ([#70](https://github.com/ikornaselur/similarium-rs/issues/70)) ([75b1c92](https://github.com/ikornaselur/similarium-rs/commit/75b1c92a344f27303858fe9bd6ed3e42e51b27ad))

## [0.4.0](https://github.com/ikornaselur/similarium-rs/compare/v0.3.0...v0.4.0) (2023-09-17)


### Features

* Add support for background workers ([#42](https://github.com/ikornaselur/similarium-rs/issues/42)) ([7e20f8f](https://github.com/ikornaselur/similarium-rs/commit/7e20f8f82fff04a68c1adddbe2761686840e0fc1))
* Add support for tracking winners and hiding secret in active games ([#63](https://github.com/ikornaselur/similarium-rs/issues/63)) ([5a4f51f](https://github.com/ikornaselur/similarium-rs/commit/5a4f51f9c12de7833e2d555b286397c29015caf5))
* Americanise words ([#68](https://github.com/ikornaselur/similarium-rs/issues/68)) ([ff543f9](https://github.com/ikornaselur/similarium-rs/commit/ff543f95e46081b8199445dc43de5d99ca406da6))
* Support ending and starting games with recurring tasks ([#67](https://github.com/ikornaselur/similarium-rs/issues/67)) ([c488064](https://github.com/ikornaselur/similarium-rs/commit/c488064f51925e1182dba4eb2f20696db5b5fb85))
* Support scheduling games for every minute of the hour ([#61](https://github.com/ikornaselur/similarium-rs/issues/61)) ([ed15eba](https://github.com/ikornaselur/similarium-rs/commit/ed15eba2746cc266b6d3e9a4567d229b15ff8a18))
* Support starting/ending games ([#50](https://github.com/ikornaselur/similarium-rs/issues/50)) ([9ca8a4b](https://github.com/ikornaselur/similarium-rs/commit/9ca8a4b7ce862f1277edc3ff70333e14535f139d))


### Miscellaneous

* Add dependabot check for github-actions ([#52](https://github.com/ikornaselur/similarium-rs/issues/52)) ([387ddac](https://github.com/ikornaselur/similarium-rs/commit/387ddac900ea6532f407f6157e425fbb52ec60bc))
* Add temporary Python script to populate database ([#46](https://github.com/ikornaselur/similarium-rs/issues/46)) ([9c218ca](https://github.com/ikornaselur/similarium-rs/commit/9c218ca6d0245cd8d7ce6a440c775ef81a45db84))
* **lint:** Run imports_granularity="Crate" manually ([#65](https://github.com/ikornaselur/similarium-rs/issues/65)) ([7dccb1b](https://github.com/ikornaselur/similarium-rs/commit/7dccb1b52d12abb082f4cb8e5bad3fd0f402c168))


### Dependencies

* bump chrono from 0.4.30 to 0.4.31 ([#58](https://github.com/ikornaselur/similarium-rs/issues/58)) ([8197325](https://github.com/ikornaselur/similarium-rs/commit/8197325ea010286e13985d817eb883d993b0cb44))
* bump serde_json from 1.0.106 to 1.0.107 ([#57](https://github.com/ikornaselur/similarium-rs/issues/57)) ([8ffd11f](https://github.com/ikornaselur/similarium-rs/commit/8ffd11f6634c3ff7a6c9eb9ba99f80b5e76e6539))
* **github-actions:** bump actions/checkout from 3 to 4 ([#55](https://github.com/ikornaselur/similarium-rs/issues/55)) ([761bac6](https://github.com/ikornaselur/similarium-rs/commit/761bac6b7d90e512324b53ea819e7cf76f69fbb1))
* **github-actions:** bump docker/build-push-action from 4 to 5 ([#56](https://github.com/ikornaselur/similarium-rs/issues/56)) ([cb01d09](https://github.com/ikornaselur/similarium-rs/commit/cb01d09800e3c6748ef51842ff8c335e66ef61a4))
* **github-actions:** bump docker/login-action from 2 to 3 ([#53](https://github.com/ikornaselur/similarium-rs/issues/53)) ([06ed0a6](https://github.com/ikornaselur/similarium-rs/commit/06ed0a6ef5cdcf605f4164f91c1da16328ab3060))
* **github-actions:** bump docker/metadata-action from 4 to 5 ([#54](https://github.com/ikornaselur/similarium-rs/issues/54)) ([a84587c](https://github.com/ikornaselur/similarium-rs/commit/a84587c3b21f7094582f2faf17e5aa1f01ac15ab))

## [0.3.0](https://github.com/ikornaselur/similarium-rs/compare/v0.2.1...v0.3.0) (2023-09-10)


### Features

* Add Dockerfile and auto publish to ghcr.io ([#43](https://github.com/ikornaselur/similarium-rs/issues/43)) ([baf8845](https://github.com/ikornaselur/similarium-rs/commit/baf8845e80340038cee0428d5986c575526ae947))
* Remove 2 and 3 letter secret words ([#38](https://github.com/ikornaselur/similarium-rs/issues/38)) ([b0c2d1b](https://github.com/ikornaselur/similarium-rs/commit/b0c2d1b3f73926cdad18705da41dcd7f424b6e78))
* Simplify game format to just show rank and word ([#25](https://github.com/ikornaselur/similarium-rs/issues/25)) ([7912470](https://github.com/ikornaselur/similarium-rs/commit/7912470eb53a1b18d91796ea14e612097dfb6064))


### Bug Fixes

* Remove duplicate target words ([f25955c](https://github.com/ikornaselur/similarium-rs/commit/f25955cd47b9a9b47684ad652278a0c5ae458e68))


### Miscellaneous

* Set dependabot to run daily with a "deps:" prefix ([d710bb8](https://github.com/ikornaselur/similarium-rs/commit/d710bb8a6b095228efe5f1cda8ffbe957ef1b30f))


### Dependencies

* bump serde_json from 1.0.105 to 1.0.106 ([#41](https://github.com/ikornaselur/similarium-rs/issues/41)) ([9a6cdc0](https://github.com/ikornaselur/similarium-rs/commit/9a6cdc09c8a02c311baa8cc2f92954966a46db6a))

## [0.2.1](https://github.com/ikornaselur/similarium-rs/compare/v0.2.0...v0.2.1) (2023-08-31)


### Miscellaneous

* Use GITHUB_TOKEN for release-please ([3a3d401](https://github.com/ikornaselur/similarium-rs/commit/3a3d401f0f1680cefb08d4373e2a5f0041196cad))


### Dependencies

* Update dependencies ([1cecf94](https://github.com/ikornaselur/similarium-rs/commit/1cecf949fbe0f1d0c2c900692629557ecb76cd5e))

## [0.2.0](https://github.com/ikornaselur/similarium-rs/compare/v0.1.0...v0.2.0) (2023-08-30)


### Features

* Add error macros ([#21](https://github.com/ikornaselur/similarium-rs/issues/21)) ([7f6c02c](https://github.com/ikornaselur/similarium-rs/commit/7f6c02c048a8bc1665b4cc26249b9e75a3d79d65))


### Bug Fixes

* Update the macros repetition to be a `+` for formatting ([#23](https://github.com/ikornaselur/similarium-rs/issues/23)) ([8d53588](https://github.com/ikornaselur/similarium-rs/commit/8d5358813a0fc46aeee14cd99e513e27a09abbe4))

## 0.1.0 (2023-08-29)


### Features

* Add basic game logic ([#6](https://github.com/ikornaselur/similarium-rs/issues/6)) ([c672954](https://github.com/ikornaselur/similarium-rs/commit/c67295439545e20ff9768aab92737b0156179a37))
