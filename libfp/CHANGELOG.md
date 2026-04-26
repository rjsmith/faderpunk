# Changelog

## 0.10.2 (2026-04-26)

### Breaking Changes

- the phoenix has risen from the ashes

### Features

- hello_world
- improve App API and add a ton of todos
- (almost) full midi over usb/uart
- deactivate eeprom for now
- deactivate mux pio for now
- add max fader change detection
- send midi message on fader change
- add ci scripts
- add ws2812 led driver
- actually implement ADC mode readings
- use atomics for thread-spanning data
- update embassy deps
- compile on rp2350
- use probe-rs next for debugging
- add button press detection + debounce
- implement proper scene sanity check
- add ButtonDown cross core message
- add ButtonDown to default app
- add XRx channels from core 1 to 0
- use transport/midi tasks instead of usb/serial
- rename to Fader Punk
- add HeroUI based suuuuper basic configurator
- use ws2812-async led driver
- implement dynamic scene changes
- temporarily disable midi
- remove async-button
- use midi2 instead of wmidi
- refactor usb driver passing
- disable websub temporarily
- re-enable midi messages
- add global variable API
- add runtime config
- add option to set fader curve
- add basic static waveforms
- add internal clock
- add external clock using aux jacks
- add clock division
- add bpm getter, improve clock precision
- add is_button_pressed, is_shift_pressed
- re-enable midi
- set_led color and brightness in api
- update embassy-rp to 0.4.0
- add lfo app
- use NoopRawMutex for MAX
- add option to define in/out ranges
- rename to faderpunk
- refactor LFO app
- run rp2350 at 250Mhz
- change lfo values, adjust gamma
- add api for top and bottom led rows
- use Watch for clock
- add simple trigger app
- add GateJack
- add reset source, scene is now layout
- add midi note on/off api
- Add dice roller
- move Waveform enum into config, add bindgen
- add gen-bindings, restructure project
- add postcard encoded app config list
- decode large configuration messages
- use batch messages for app listing
- show params in configurator temp page
- (very) simple button debounce
- add mute led to default app
- redesign app parts, restructure waiters
- add button debounce, long press
- improve lfo
- add wait_for_any_long_press function to app
- refactor leds a bit, add chan clamping
- refactor midi into struct
- make midi channel configurable in default app
- add sequential storage using eeprom
- use StorageSlots for app storage values
- simplify cross core message routing
- add AppParams macro and storage
- ParamStore -> Store, impl ser and des for Store
- StorageSlot is now dependent on Store
- store and recall current values using rpc
- add app cleanup method
- move param handler into param store
- add param and cleanup loops to all apps
- store GlobalConfig in FRAM
- set clock sources using the configurator
- add and set params for apps
- re-spawn apps on param change
- add param load and save for apps
- use ClockEvent instead of bool for clock Watch
- make max and midi channels CriticalSectionRawMutex Channels
- use PubSubChannel for clock
- vastly improve Storage API
- restructure Arr and AppStorage
- add midi input message forwarding
- use static buffer for fram reads
- remove release-plz workflow
- add modify method to Global
- add usb windows compatibility
- refactor leds to allow for effects
- move BrightnessExt to libfp
- add led overlay effects and flash effect
- add temporary scene save and recall effects
- merge config crate into libfp
- add value transformation for Enum/usize
- update all dependencies
- add -5V to 5V range to manual calibration
- move Range to libfp
- improve semi-automatic calibration
- add color parameters to most apps
- select color and icons for all app. Rework app order
- migrate main configurator deployment to gh-pages branch
- add beta release workflow for develop branch
- add possibility to save and recall app layouts & params
- bump minimum version to 1.5.0
- add specialized midi params
- add unique USB serial numbers from RP2350 chip ID
- add jump and scale latch pickup modes
- add takeover modes to manual (#437)
- show led info on scene button press (#489)
- add random+ app (#453)
- add clock divider+ app  (#456)
- add fp-grids app (#467)
- add swing option to clock (#491)

### Fixes

- use color order as marker struct
- remove Option from DAC values
- move jack configuration state to max
- use timeout for usb midi message
- use atomics instead of channels for fader move event
- a little bit of clean up
- basic cross core comms working
- improve cross core comms, implement waiter
- adc channel numbering
- use slice for scene set message
- send clock signal to all channels
- proper channel assignment of (In|Out)Jacks
- handle uart rx error, remove some logs
- midi cc count
- waveform saw to u16
- remove superfluous ImageDef
- flashing bug
- shift is 17
- improve compiler optimization settings
- immediately set led atomics
- implement internal clock using Ticker
- mute midi in default app
- make clock work using MAX GPO ports
- use permanent receiver for clock
- clock fixes and clock debug app
- serialize large arrays
- use Signal instead of Watch for ParamStore
- alter macro to account for apps without params
- restructure GlobalConfig to be Serialize, Deserialize
- wait for fram to be ready on startup
- midi uart message drops
- loading of Globalconfig
- move build profiles to workspace
- drop guard for storage before saving
- potential mutex deadlocks
- use correct mutex type for FRAM buffers
- sequentialize FRAM reads and writes
- use read buffer pool for fram reads
- use stack buffer for fram reads for callers
- use direct memory access fram read buffers
- vscode rust-analyzer settings
- scene 0 should not recall "current" values
- update postcard-bindgen to non-fork version
- fix recurring mistake when using ticks
- housekeeping
- actually respond to i2c read requests
- make a change to force rebuild
- do not panic in app macro functions
- validate layout after loading from fram
- prefixed commit
- 1 bar division  was wrong
- exponential and logarithmic curves were switched
- fix subdivision numbers
- ensure gh-pages deployment pushes to correct branch
- never erase calibration range
- add hardware factory reset
- double usb MAX_PAYLOAD_SIZE to 512 bytes
- remove unused navigate parameter from connect function
- clippy issues

## 0.10.2-beta.0 (2026-04-17)

### Features

- show led info on scene button press (#489)
- add random+ app (#453)
- add clock divider+ app  (#456)
- add fp-grids app (#467)
- add swing option to clock (#491)

## 0.10.1 (2026-02-23)

### Features

- add jump and scale latch pickup modes
- add takeover modes to manual (#437)

## 0.10.1-beta.0 (2026-02-11)

### Features

- add jump and scale latch pickup modes

## 0.10.0 (2026-02-08)


### Features

* add specialized midi params ([b288f57](https://github.com/ATOVproject/faderpunk/commit/b288f5720255efaf136464408599725ac0be9adf))
* **midi:** midi out routing ([9d773b1](https://github.com/ATOVproject/faderpunk/commit/9d773b1806c5dbd52df542d4363190df5460756f))
* **quantizer:** add ability to get current scale in apps ([d421562](https://github.com/ATOVproject/faderpunk/commit/d42156201d3285cd4e3ea67002dfcb2d9afe041b))


### Bug Fixes

* **led:** increase dynamic range, simplify nomenclature ([0e31ec7](https://github.com/ATOVproject/faderpunk/commit/0e31ec763b87c831eba9ef241cc8ba6850e71987))
* **slew & follower:** extend slew range, added passthrough a minimum ([18f64cc](https://github.com/ATOVproject/faderpunk/commit/18f64cc6c8e078a5d16f7d1338a2d82fb1b3a976))

## 0.9.0 (2025-12-07)


### Features

* add possibility to save and recall app layouts & params ([e724ac0](https://github.com/ATOVproject/faderpunk/commit/e724ac087893a3beb53ea4e4f39449dbb238ea68))
* **calibration:** switch to fully automatic calibration ([f88604d](https://github.com/ATOVproject/faderpunk/commit/f88604d641a5aa1b2d2998f12faee275d71950f2))
* **configurator:** add factory reset function ([85dc047](https://github.com/ATOVproject/faderpunk/commit/85dc047d3a8b463a746925df9de9c8778841c739))
* **configurator:** add Key and Tonic mapping to the manual ([feeecf0](https://github.com/ATOVproject/faderpunk/commit/feeecf0016221613179044eb81c7932c2f2e023a))
* **panner:** add panner app ([d990c75](https://github.com/ATOVproject/faderpunk/commit/d990c752e5bdd51704a1323dfc798c50b7ba33e6))

## 0.8.2 (2025-10-24)


### Bug Fixes

* **control:** actually fix the bug causing CC not reaching 127 ([7cb3288](https://github.com/ATOVproject/faderpunk/commit/7cb32889f703d1b91c37b0da3318c1c559d80623))
* **control:** fix CC output not reaching 127 ([6c9baec](https://github.com/ATOVproject/faderpunk/commit/6c9baec9d2f47028ef23e492009cbb9de720eb1a))

## 0.8.1 (2025-10-08)


### Bug Fixes

* **quantizer:** increase codebook size for increased range ([291f35d](https://github.com/ATOVproject/faderpunk/commit/291f35da6f18adc0b2dfe52c8ed23b16ac0b32e4))

## 0.8.0 (2025-09-25)


### Features

* **configurator:** rename params, fix float field ([9349aa6](https://github.com/ATOVproject/faderpunk/commit/9349aa624432e3aef66b71a7a1a19e2b40dacef8))


### Bug Fixes

* **clock:** limit extra reset sources ([7fc8619](https://github.com/ATOVproject/faderpunk/commit/7fc861910648376d5f7963214c1c6f2a33df7bd5))

## 0.7.0 (2025-09-20)


### Features

* **configurator:** new app overview, get and set app params ([06bf6c3](https://github.com/ATOVproject/faderpunk/commit/06bf6c338f6abd07688952d88dcebd06dfadb8c6))


### Bug Fixes

* **configurator:** retain storage and parameters when app is moved ([6ea3cab](https://github.com/ATOVproject/faderpunk/commit/6ea3cab3c1e5ae7a8213c57c10b453972b2b48c0))

## 0.6.0 (2025-09-13)


### Features

* **app:** add color and icon config to apps ([35d19f9](https://github.com/ATOVproject/faderpunk/commit/35d19f92412597c0cb090c60d2c2ed06b4688342))
* **calibration:** move to fixed point calibration ([574d899](https://github.com/ATOVproject/faderpunk/commit/574d89908ff705ab428a040bd6e8b095978e82ee))
* **clock:** add analog clock out from internal clock ([7c3b619](https://github.com/ATOVproject/faderpunk/commit/7c3b619545862a5e22bd65f07dd9c37c0e3ca7c4))
* **clock:** add really long clock divisions ([1a15f70](https://github.com/ATOVproject/faderpunk/commit/1a15f70c0bf96e1b3351e92d5a31a69c9084b6df))
* **clock:** add reset out aux config option ([d021133](https://github.com/ATOVproject/faderpunk/commit/d02113302ed7f3cd45837acd013ff6b35e96eb3c))
* **configurator:** add note param (in case we need it) ([22f50b3](https://github.com/ATOVproject/faderpunk/commit/22f50b368c90d894d3ee6f791fe342f732906b52))
* **configurator:** add range param ([f5014a0](https://github.com/ATOVproject/faderpunk/commit/f5014a0ee0a53ffa0102d5e39f9750813ebf2ef6))
* **i2c:** i2c leader (16n compatibility mode) ([0123546](https://github.com/ATOVproject/faderpunk/commit/012354629fbc6462891a9df604250e9fa34cbea4))
* **layout:** make 16x control the default layout ([3480017](https://github.com/ATOVproject/faderpunk/commit/3480017b2a1748393b308712e8b974cb6e0438dc))
* select color and icons for all app. Rework app order ([e97a390](https://github.com/ATOVproject/faderpunk/commit/e97a390490ff0f9187f809f8231f308718efab98))


### Bug Fixes

* **clock:** prevent drift and stutter while changing bpm ([da6af19](https://github.com/ATOVproject/faderpunk/commit/da6af19d93e2e8b9ac3bd4814442ac2bfdda9238))
* exponential and logarithmic curves were switched ([6a2f311](https://github.com/ATOVproject/faderpunk/commit/6a2f3111712a0cb3a993bf2fae294efe3a6667bf))
* **quantizer:** use the first 16 scale from o_C ([9a0e5c7](https://github.com/ATOVproject/faderpunk/commit/9a0e5c7ae073458aee048ab5aa3ddba1b1bb5131))

## 0.5.0 (2025-09-04)


### Features

* **api:** complete rework of app parameter implementation ([8a6f44d](https://github.com/ATOVproject/faderpunk/commit/8a6f44dcefe066d20a1db0e81c96a3fa3caa1832))
* **api:** new color api and improved color consistency ([056761f](https://github.com/ATOVproject/faderpunk/commit/056761ff42a336f8836da01ec7a58c773b6e5598))
* **api:** use new latch in default and lfo ([9abe1cd](https://github.com/ATOVproject/faderpunk/commit/9abe1cdf27c78bd8dfc72cb3b2b946b15d2ea95d))
* **config:** add ability to change global config via faders ([9374b77](https://github.com/ATOVproject/faderpunk/commit/9374b779429b8bb6f242dca0ae5078368ad4ecd5))
* **config:** introduce more global settings, config task loop ([17e48d4](https://github.com/ATOVproject/faderpunk/commit/17e48d4a9f1fcf43130984e9adaa0505c5e2dae6))
* **cv2midi:** add cv2midi app ([ab4864f](https://github.com/ATOVproject/faderpunk/commit/ab4864f3714c9907dc485aaf77893b2ae5cd3d09))
* **euclid:** add euclid app ([2eaff71](https://github.com/ATOVproject/faderpunk/commit/2eaff715aacbcf0c1e643768ce9a9cf8348f67e4))
* **latch:** add third latch layer ([c827400](https://github.com/ATOVproject/faderpunk/commit/c82740005ad0829f4fd7eee9ef80a01389dc23cf))
* **leds:** improve led brightness and color apis ([4ff24e2](https://github.com/ATOVproject/faderpunk/commit/4ff24e20b812fcbcaa332297c126f82b072e2848))
* **quantizer:** rewrite quantizer, make it more predictable ([0c14ef6](https://github.com/ATOVproject/faderpunk/commit/0c14ef6f9d8561f74b9b85f157de4acbeaf19c08))


### Bug Fixes

* **clock:** adjust clock config only when it was changed ([9d53f36](https://github.com/ATOVproject/faderpunk/commit/9d53f36edf53b4cd33089df2a9dac831d012eab1))
* **euclid:** change description ([549611e](https://github.com/ATOVproject/faderpunk/commit/549611e2aa7ba9f721e04164a477ca0f5d0e58fa))
* **latch:** add jitter tolerance to latch ([95f4c67](https://github.com/ATOVproject/faderpunk/commit/95f4c67167bf42f58413b5635404e048dcc39818))
* **latch:** unlatch when target value is changed externally ([b2caa67](https://github.com/ATOVproject/faderpunk/commit/b2caa67eda5c78b3e695b51ad33dfdf7ccc95ad7))
* **layout:** allow for holes in layout ([20ff5bc](https://github.com/ATOVproject/faderpunk/commit/20ff5bc92461369f145b13716ba3fe45f93e3e4c))
* validate layout after loading from fram ([848e2aa](https://github.com/ATOVproject/faderpunk/commit/848e2aa79130d134737f66309c023211e041f861))

## 0.4.0 (2025-08-23)


### Features

* **midi2cv:** add midi2cv prototype app ([c005ac1](https://github.com/ATOVproject/faderpunk/commit/c005ac1c0d0d7b4827dcde9ff5f7a7057a3b015f))


### Bug Fixes

* **calibration:** fixes for semi-automatic calibration ([932321b](https://github.com/ATOVproject/faderpunk/commit/932321bad07da39aaa704c64fcc023f7399ea835))

## 0.3.0 (2025-08-20)


### Features

* add -5V to 5V range to manual calibration ([f6cee85](https://github.com/ATOVproject/faderpunk/commit/f6cee85878316bb552e7ba28f405bb2b6b556fcb))
* **calibration:** add first version of automatic calibration ([2679d6b](https://github.com/ATOVproject/faderpunk/commit/2679d6b955d5b2e50e9ac3028050ecac5450f90a))
* **calibration:** move manual calibration to i2c startup ([83a0c03](https://github.com/ATOVproject/faderpunk/commit/83a0c03e97c0fba81c4545b0734cb066556f4e1e))
* **config:** separate layout from global config ([54d8690](https://github.com/ATOVproject/faderpunk/commit/54d869014c2299812519a4b47cc0b8a9a069a09f))
* **i2c:** prepare for i2c leader/follower/calibration modes ([2269d84](https://github.com/ATOVproject/faderpunk/commit/2269d841e35dd07a73397bd2a234977b944e2fc7))
* improve semi-automatic calibration ([71d1f4e](https://github.com/ATOVproject/faderpunk/commit/71d1f4e46590adc99d62477ad577860ae5554331))
* move Range to libfp ([a349b55](https://github.com/ATOVproject/faderpunk/commit/a349b55924c98180409e89da698f7b392b2b9323))
* **params:** add Color param for configurator ([a7b2ee6](https://github.com/ATOVproject/faderpunk/commit/a7b2ee65cca6d0047b82097bace0d895a24ce4d2))
* **params:** bump app max param size to 8 ([7900abc](https://github.com/ATOVproject/faderpunk/commit/7900abc2e749ac0311d6d2100eb5ed8b6c865325))
* **params:** use .into() instead of .get() for Color ([818391b](https://github.com/ATOVproject/faderpunk/commit/818391b30f2e99d281965a63a27f0e84031ead7b))

## 0.2.2 (2025-08-14)


### Bug Fixes

* **default:** fix curve, slew and bipolar recall ([968d4df](https://github.com/ATOVproject/faderpunk/commit/968d4dfca3812f1f3f4084d8a9448b81b70a7603))

## 0.2.1 (2025-08-08)


### Bug Fixes

* **api:** rename Sawinv to SawInv ([9b18e3c](https://github.com/ATOVproject/faderpunk/commit/9b18e3c5f6fd4134e83119d209608b06f5a863e0))

## 0.2.0 (2025-08-08)


### Features

* **constants:** introduced some standard LED colors and intensities ([2e2baa3](https://github.com/ATOVproject/faderpunk/commit/2e2baa3f92c27a83cb1f276791162070a4610914))
* **deps:** downgrade heapless in libfp ([8ee90ca](https://github.com/ATOVproject/faderpunk/commit/8ee90ca18c7aa34a187fcea6edf41f057809765a))
* **lfo:** add inverted saw waveform ([b32c7bc](https://github.com/ATOVproject/faderpunk/commit/b32c7bc923010eb65ae5a7ba5b0072cf674aebc5))
* **quantizer:** add quantizer utility ([201a47b](https://github.com/ATOVproject/faderpunk/commit/201a47b3dc9beeaefd57f0f84931c4565e129385))
* **sequencer:** add legato ([e60b3ea](https://github.com/ATOVproject/faderpunk/commit/e60b3ea0cc56dc7d0d5663d92db181f37b6a761f))
* **utils:** add clickless function as public ([819042b](https://github.com/ATOVproject/faderpunk/commit/819042b4f788d795168c841473c8dd4ca56fc96b))


### Bug Fixes

* **constants:** adjust rgb to design guide value ([7e47192](https://github.com/ATOVproject/faderpunk/commit/7e47192926c1e4a0db9fcb3bb31059befad5d838))

## 0.1.0 (2025-07-19)


### Features

* add gen-bindings, restructure project ([0628406](https://github.com/ATOVproject/faderpunk/commit/06284069ff090d442f921713c12f794181328aab))
* **calibration:** add output calibration over i2c ([d8b25a1](https://github.com/ATOVproject/faderpunk/commit/d8b25a1d09294f39396d8960110223bdc71d24a6))
* **calibration:** i2c proto ping pong ([2c1d190](https://github.com/ATOVproject/faderpunk/commit/2c1d190ccb7a76c5bc61cc96cae9749a6277a833))
* **configurator:** implement layout setting ([17cb7b3](https://github.com/ATOVproject/faderpunk/commit/17cb7b338c8764302ada0ed4b54e7c74fbd5e2db))
* **libfp:** add value transformation for Enum/usize ([354cc98](https://github.com/ATOVproject/faderpunk/commit/354cc9854b99208b14a8df37b6a34a3a1d556972))
* merge config crate into libfp ([d69da45](https://github.com/ATOVproject/faderpunk/commit/d69da45ed8b4a60fd020ce567328b348cf475319))
* move BrightnessExt to libfp ([0972e2d](https://github.com/ATOVproject/faderpunk/commit/0972e2d192cc615ebb831a273bf71dedaa7c2af0))
* simplify cross core message routing ([7030d14](https://github.com/ATOVproject/faderpunk/commit/7030d14cc1027c85a48fc73501f91bbe267496bb))
* **utils:** add attenuverter and slew_limiter ([c1c30f0](https://github.com/ATOVproject/faderpunk/commit/c1c30f071c615727c122f4d5196ce5448689ff31))
* **utils:** introduce some useful functions ([26432b4](https://github.com/ATOVproject/faderpunk/commit/26432b4f7b922dd988da904411f4d00642fcb1a3))


### Bug Fixes

* clock fixes and clock debug app ([2f39258](https://github.com/ATOVproject/faderpunk/commit/2f392588048dae6c361383c2fe4aac4ee508c464))
* **midi:** proper midi 1 implementation using midly ([ea38aca](https://github.com/ATOVproject/faderpunk/commit/ea38aca53bb42330f03e86fbb0a78933aeedeb91))
* **utils:** add clamps to spliters ([77301e1](https://github.com/ATOVproject/faderpunk/commit/77301e12ecb98787822de16729c31c17a60318b1))
