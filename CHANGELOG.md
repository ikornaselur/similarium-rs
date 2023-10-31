# Changelog

## [0.6.5](https://github.com/ikornaselur/similarium-rs/compare/v0.6.4...v0.6.5) (2023-10-31)


### Miscellaneous

* Add codecov to test action ([#115](https://github.com/ikornaselur/similarium-rs/issues/115)) ([2f25052](https://github.com/ikornaselur/similarium-rs/commit/2f250520ebabceca16f1760b46a54c216fd933af))
* Replace grcov with llvm-cov ([#118](https://github.com/ikornaselur/similarium-rs/issues/118)) ([14f0d00](https://github.com/ikornaselur/similarium-rs/commit/14f0d0006d87b77360768536721d4140bb02161b))
* Skip running linting and testing on tag creation ([#114](https://github.com/ikornaselur/similarium-rs/issues/114)) ([53e2211](https://github.com/ikornaselur/similarium-rs/commit/53e2211397fd1acfa8541322a15eda87a0d54120))
* **test:** Introduce mockito/mockall and increase test coverage ([#117](https://github.com/ikornaselur/similarium-rs/issues/117)) ([11b2561](https://github.com/ikornaselur/similarium-rs/commit/11b25616d87c81541a53c914580f685018c9fd63))
* Tweak codecov further and add basic tests ([#119](https://github.com/ikornaselur/similarium-rs/issues/119)) ([f3f538e](https://github.com/ikornaselur/similarium-rs/commit/f3f538e1eb4b12c379c3dbf66eb8e80c0d0d6c39))


### Dependencies

* bump serde from 1.0.189 to 1.0.190 ([#121](https://github.com/ikornaselur/similarium-rs/issues/121)) ([c7fb873](https://github.com/ikornaselur/similarium-rs/commit/c7fb8738cdf27857d24e90238849df02eee46ede))
* bump serde_json from 1.0.107 to 1.0.108 ([#122](https://github.com/ikornaselur/similarium-rs/issues/122)) ([c218b41](https://github.com/ikornaselur/similarium-rs/commit/c218b414183aa8283c928297d058e25fd341c426))
* bump uuid from 1.4.1 to 1.5.0 ([#120](https://github.com/ikornaselur/similarium-rs/issues/120)) ([03596b7](https://github.com/ikornaselur/similarium-rs/commit/03596b7f58074c8b33c8614d89cc54abc6139810))

## [0.6.4](https://github.com/ikornaselur/similarium-rs/compare/v0.6.3...v0.6.4) (2023-10-14)


### Miscellaneous

* Add a nightly task to clean up materialised views ([#112](https://github.com/ikornaselur/similarium-rs/issues/112)) ([8565d43](https://github.com/ikornaselur/similarium-rs/commit/8565d437d33c9b0df882e92a5860d5c8b9831125))

## [0.6.3](https://github.com/ikornaselur/similarium-rs/compare/v0.6.2...v0.6.3) (2023-10-14)


### Miscellaneous

* **CI/CD:** Continously deploy releases to prod ([#111](https://github.com/ikornaselur/similarium-rs/issues/111)) ([a025cfe](https://github.com/ikornaselur/similarium-rs/commit/a025cfe318e1b5225126192784a41b2c52a90a50))
* **CI/CD:** Trigger staging deployment after pushing :dev image ([#109](https://github.com/ikornaselur/similarium-rs/issues/109)) ([22f4f4d](https://github.com/ikornaselur/similarium-rs/commit/22f4f4d5f1acddfa347b174af41cd08d4d043edb))


### Dependencies

* bump serde from 1.0.188 to 1.0.189 ([#108](https://github.com/ikornaselur/similarium-rs/issues/108)) ([2700425](https://github.com/ikornaselur/similarium-rs/commit/2700425498967b1ab61ab6fa0d581e66bdc5014d))

## [0.6.2](https://github.com/ikornaselur/similarium-rs/compare/v0.6.1...v0.6.2) (2023-10-13)


### Bug Fixes

* Set the fallback text to the header text ([#106](https://github.com/ikornaselur/similarium-rs/issues/106)) ([43ea620](https://github.com/ikornaselur/similarium-rs/commit/43ea620f7888d58dc886a16b484653f4b6b4833f))

## [0.6.1](https://github.com/ikornaselur/similarium-rs/compare/v0.6.0...v0.6.1) (2023-10-10)


### Bug Fixes

* Correctly set the guess num for winners ([#102](https://github.com/ikornaselur/similarium-rs/issues/102)) ([c0bd1e8](https://github.com/ikornaselur/similarium-rs/commit/c0bd1e8ac246cc3e1073049adf870d1f261248a6))
* Fix issue with latest guesses showing original instead of latest guesser ([#101](https://github.com/ikornaselur/similarium-rs/issues/101)) ([6660898](https://github.com/ikornaselur/similarium-rs/commit/666089819390cb1d30d0694024b1980b2ae66728))
* Fix manual ending not ending inactive games ([#103](https://github.com/ikornaselur/similarium-rs/issues/103)) ([b81ab26](https://github.com/ikornaselur/similarium-rs/commit/b81ab26db6b3ba9fac5268ab754a8002d06ed10c))


### Dependencies

* bump tokio from 1.32.0 to 1.33.0 ([#96](https://github.com/ikornaselur/similarium-rs/issues/96)) ([f1dab5f](https://github.com/ikornaselur/similarium-rs/commit/f1dab5feffcbcd43eb0525643b6bb972d262d120))

## [0.6.0](https://github.com/ikornaselur/similarium-rs/compare/v0.5.1...v0.6.0) (2023-10-08)


### Features

* Only end games automatically if there are no guesses ([#94](https://github.com/ikornaselur/similarium-rs/issues/94)) ([48c9d09](https://github.com/ikornaselur/similarium-rs/commit/48c9d09b481c656da3596689df82c959e1e2aca1))
* Reveal secret and winners at the end ([#93](https://github.com/ikornaselur/similarium-rs/issues/93)) ([8beff0f](https://github.com/ikornaselur/similarium-rs/commit/8beff0fa33b8557e1df32f61bfafce28cf164dcf))


### Bug Fixes

* Trim guess inputs ([#91](https://github.com/ikornaselur/similarium-rs/issues/91)) ([15a1d7c](https://github.com/ikornaselur/similarium-rs/commit/15a1d7cd55fc46e7f626e31b246826363bcb9bbe))

## [0.5.1](https://github.com/ikornaselur/similarium-rs/compare/v0.5.0...v0.5.1) (2023-10-07)


### Bug Fixes

* Fix issue with winning guess erroring ([#87](https://github.com/ikornaselur/similarium-rs/issues/87)) ([3bdbaf0](https://github.com/ikornaselur/similarium-rs/commit/3bdbaf0bd3fec2339f49b22c8e19cf848e8f2ff2))


### Miscellaneous

* Fix dockerfile COPY entry ([#86](https://github.com/ikornaselur/similarium-rs/issues/86)) ([f0b19e7](https://github.com/ikornaselur/similarium-rs/commit/f0b19e7e8ce0c72cb131e8cd6f60f323dbd7eed7))
* Use cargo build instead of cargo install ([#84](https://github.com/ikornaselur/similarium-rs/issues/84)) ([44411fd](https://github.com/ikornaselur/similarium-rs/commit/44411fd88dfc1d2a84fe9faca53b90b2b790d7b6))

## [0.5.0](https://github.com/ikornaselur/similarium-rs/compare/v0.4.3...v0.5.0) (2023-10-06)


### Features

* Post ephemeral back to user if word is unknown ([#82](https://github.com/ikornaselur/similarium-rs/issues/82)) ([c4199d5](https://github.com/ikornaselur/similarium-rs/commit/c4199d540a71a712abc21e3f1c3c2e3cfa247c6c))


### Miscellaneous

* Tweak the dockerfile to include ca-certificates ([#80](https://github.com/ikornaselur/similarium-rs/issues/80)) ([ac43c1f](https://github.com/ikornaselur/similarium-rs/commit/ac43c1fe1519b054d21902c5eb9185bc86c415b7))


### Dependencies

* bump reqwest from 0.11.20 to 0.11.21 ([#78](https://github.com/ikornaselur/similarium-rs/issues/78)) ([ca04ac6](https://github.com/ikornaselur/similarium-rs/commit/ca04ac67bec62d2d67b15e0ac9058409ff470688))
* bump reqwest from 0.11.21 to 0.11.22 ([#79](https://github.com/ikornaselur/similarium-rs/issues/79)) ([2bfd8a5](https://github.com/ikornaselur/similarium-rs/commit/2bfd8a5b48858287f3b60b74fd9d39feec2cbc7b))
* bump sha2 from 0.10.7 to 0.10.8 ([#75](https://github.com/ikornaselur/similarium-rs/issues/75)) ([2338646](https://github.com/ikornaselur/similarium-rs/commit/233864606442efea25e0269cd5dc3c5c4c49894d))
* bump sqlx from 0.7.1 to 0.7.2 ([#76](https://github.com/ikornaselur/similarium-rs/issues/76)) ([641d6cb](https://github.com/ikornaselur/similarium-rs/commit/641d6cb705f4867b058f87baf5c071e131aafa44))

## [0.4.3](https://github.com/ikornaselur/similarium-rs/compare/v0.4.2...v0.4.3) (2023-09-17)


### Miscellaneous

* **actions:** Change trigger for publish to be just on actual tags ([1ee0ad1](https://github.com/ikornaselur/similarium-rs/commit/1ee0ad1e4e5283dabeda5e4860059e328d6ed17e))

## [0.4.2](https://github.com/ikornaselur/similarium-rs/compare/v0.4.1...v0.4.2) (2023-09-17)


### Miscellaneous

* **actions:** Set checkout in publish-docker to fetch tags ([b4aad97](https://github.com/ikornaselur/similarium-rs/commit/b4aad97734e199935e3e0ca94fe9860c5e1bcf40))

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
