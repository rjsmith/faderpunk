# Changelog

## 0.10.1 (2026-02-13)

### Breaking Changes

- the phoenix has risen from the ashes
- release configurator 1.0

### Features

- hello_world
- improve App API and add a ton of todos
- (almost) full midi over usb/uart
- deactivate eeprom for now
- deactivate mux pio for now
- add max fader change detection
- send midi message on fader change
- add led state machine for simple effects
- add proper fader waiter and blink led
- add preliminary channel button presses
- establish basic webusb connection
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
- send custom cc value
- set shift and scene button to white
- add MidiIn and MidiUSB clock sources
- add glitchy startup animation
- add sequential storage using eeprom
- use StorageSlots for app storage values
- allow storing arrays
- simplify cross core message routing
- add AppParams macro and storage
- retrieve app state from configurator
- set a param from configurator
- always require params() in config macro
- move storage globals into app_config
- add simple scene implementation for StorageSlots
- integrate scenes with scene button
- add wait_for_scene_change method
- pre-load everything from eeprom
- read-before-write
- allow long arrays for storage slots
- ParamStore -> Store, impl ser and des for Store
- StorageSlot is now dependent on Store
- store and recall current values using rpc
- implement layout setting
- set custom layouts
- deploy to Github pages
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
- use Signals instead of Channel
- set fader refresh rate to 1ms
- load calibration data, use it in max task
- i2c proto ping pong
- add output calibration over i2c
- add ability to use effects in apps
- add manual calibration app
- add latest seq8 version
- add latest automator version
- add latest lfo version
- add latest ad version
- remove test apps
- add latest version of probatrigger app
- add latest version of turing app
- add latest version of clkturing app
- improve Curve api
- add latest version of slew app
- add latest version of follower app
- improve fader api
- introduce some useful functions
- add attenuverter and slew_limiter
- merge config crate into libfp
- add value transformation for Enum/usize
- return is_shift_pressed from any button press
- add undo for the last calibration step
- update all dependencies
- improve die roll function signature
- remove usb logging for now
- use auto-generated device version
- fix webusb windows compatibility
- add quantizer utility
- add MIDI CC selection
- add clickless mute, remove stepping
- add bipolar param
- add bipolar and curve param
- add clickless function as public
- add fader curve param for testing
- add attenuation
- add attenuation
- introduced some standard LED colors and intensities
- add quantizer to apps
- add inverted saw waveform
- add clocked mode
- add legato
- add rgb test app
- downgrade embassy-executor and embassy-rp for now
- downgrade heapless in libfp
- add MidiOutput, MidiInput, MidiDuplex APIs
- add notefader app
- add Color param for configurator
- bump app max param size to 8
- use .into() instead of .get() for Color
- add Color param component in configurator
- separate layout from global config
- prepare for i2c leader/follower/calibration modes
- add -5V to 5V range to manual calibration
- move manual calibration to i2c startup
- add first version of automatic calibration
- move Range to libfp
- improve semi-automatic calibration
- make configurator releases with built artifacts
- add midi2cv prototype app
- add mute and led feedback
- reset messages resets the LFO
- add mute on release proto function
- add color parameters to most apps
- add trigger on button, add midi trigger, add trigger to gate
- rewrite quantizer, make it more predictable
- introduce more global settings, config task loop
- add ability to change global config via faders
- improve led brightness and color apis
- add button release API
- globals are now sync
- use new latch in default and lfo
- complete rework of app parameter implementation
- add Offset+Attenuator app
- add third latch layer
- new color api and improved color consistency
- add slew, refine LEDs
- renamed to "Control", make curve symmetrical around 0 when bipolar
- rename default to control
- bring back startup animation
- colorize scene and shift button
- add euclid app
- add quantizer app
- add cv2midi app
- add cv2midinote app
- add octave and semitone shift, add mute, add led feedback
- add offset toggles
- add semitone offset toggle
- add quantizer on input
- add input gain control
- increase max gain to 2x
- add range param
- add note param (in case we need it)
- use enum for midi modes in midi2cv and turing
- i2c leader (16n compatibility mode)
- add midi aftertouch and pitch bend API
- move to fixed point calibration
- start internal clock with scene+shift
- send midi clock ticks when using internal clock
- passthrough midi clock usb<->uart
- remove all midi passthrough
- add crc check
- add color and icon config to apps
- select color and icons for all app. Rework app order
- add analog clock out from internal clock
- refactor clock to allow for improved routing
- add really long clock divisions
- add reset out aux config option
- add range option
- add octave selection in shift functions
- add gate on note mode
- add the ability to deactivate the midi input
- add use_midi param
- make 16x control the default layout
- add gate indicator
- add new configurator scaffold
- add new device page
- new app overview, get and set app params
- app layout drag&drop
- store layout on device
- add saved confirmation
- add possibility to remove apps
- add modal to add apps
- remove old configurator
- save global settings
- connect page, minor additions
- rename params, fix float field
- add base note and gate % param
- add speed  param
- add invert param
- add slew to CV and bipolar param
- add free running mode
- change clkcvrnd name to rndcvcc
- add manual template
- add manual page
- manual app style improvements
- display update message
- add initial version of all app manuals
- add update guide and fw link
- add mvm
- add button to clear apps
- add param enabling fader value storage
- add range selection, add MIDI output
- make buttons toggles of offset and attenuverter
- update app manual for v1.4.0
- migrate main configurator deployment to gh-pages branch
- add beta release workflow for develop branch
- add range param
- add base note param
- update Turing and Turing+ manual
- add panner app
- add clock divider app
- add possibility to save and recall app layouts & params
- start with running clock & save clock state
- add factory reset function
- add velocity to `Gate` and `Note Gate` mode
- send Out1 copy to MIDI Out2
- store config with layout in setup file
- add Key and Tonic mapping to the manual
- switch to fully automatic calibration
- bump minimum version to 1.5.0
- bump version to align with configurator
- add specialized midi params
- midi out routing
- add tb-303 style glide functionality
- add MIDI retrigger param
- add range param
- add unique USB serial numbers from RP2350 chip ID
- add velocity to gate param
- add CC button modes
- add lfo+ app
- add ability to get current scale in apps
- add documentation for v1.7 midi functions (#416)
- add reset cv destination (#415)
- add jump and scale latch pickup modes

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
- improve clock reset behavior
- proper midi 1 implementation using midly
- serialize large arrays
- quick fix for midi tx over uart. remove running status
- use Signal instead of Watch for ParamStore
- alter macro to account for apps without params
- improve scene save debounce
- raise storage bytes limit
- check in pnpm-lock.yaml
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
- scene load/save debounce
- fix param fetch
- extend scenes to 16
- only update LEDs at actual refresh rate
- run led tasks in parallel
- add separate channel for led overlay effects
- remove saving
- rename
- full rework
- some led changes
- change name and description
- full rework
- full rework
- add clamps to spliters
- refine
- refine
- use utils functions
- fix led and latching bug
- make it recall resolution and gate length on boot
- add attenuation fix leds
- full rework
- added param, saving and led feedback
- fix recurring mistake when using ticks
- fix bug where curve was not properly applied
- fix  a little LED bug
- full rework
- add led feedback
- correct input value properly
- add a little more leeway for the calibration range
- make READ_BUFFERS pointer cast a bit more ideomatic
- make bit flip more direct
- kill previous buffer before recording a new one
- add interpolation to remove stepping
- bigger dead zones on probability
- housekeeping
- CH4 not recalling gate and resolution
- adjust rgb to design guide value
- disable quantizer for now
- fix crash on certain fader positions
- rename Sawinv to SawInv
- improve debounce and add button state sync
- fix crashes on certain recalled values
- remove running light when stopped
- change offset default value to 0
- use the common red color value
- small changes to make it easier to change LED colors
- fix curve, slew and bipolar recall
- fix bipolar recall, update slewing to new method
- reactivate quantizer
- change clock switch procedure
- small led fix, fix note trigger when changing mode
- load params at app startup
- better read buffer error handling
- increase uart rx buffer size
- improve midi subscriber instantiation
- properly handle larger usb midi packets
- make wait_for_message method public
- actually respond to i2c read requests
- add I2cMode to gen-bindings
- adjust for i2c global params
- make a change to force rebuild
- adjust transformValues for 8 params
- add linker config
- fix color param not being sent
- fixes for semi-automatic calibration
- improve midi subscriber instantiation
- adjust color order for consistency
- fix small led bug
- allow for holes in layout
- do not panic in app macro functions
- validate layout after loading from fram
- refresh layout after setting it
- send correct layout with channel sizes
- unlatch when target value is changed externally
- adjust minimum led brightness
- scale fader readings across the dead zone
- add all colors to configurator
- add jitter tolerance to latch
- remove automator from app list
- implement new latching system
- adjust clock config only when it was changed
- update to new new latching
- update to new latching system
- upgrade to new latching system
- update to new latching system
- implement new latching system, refine code
- implement new latching system
- implement new latching system
- fix led
- fix midi notes
- change description
- add description
- fix consistency issue between midi output and V/oct
- implement new latching system
- add description
- modify name and description
- prefixed commit
- remove CC param
- remove fader curve param
- bug upon changing the resolution
- 1 bar division  was wrong
- Fix default values on rotation and randomization
- fix params not being applied
- rework to implement new button API
- fix division setting not being recalled
- fix led brightness inconsistency
- reduce slew on mute
- fix issue crash when changing to certain params
- set offset to correct default value
- fix attenuation curve
- use the first 16 scale from o_C
- add initialization check
- use stream parsing for cobs frames
- fix startup button press calibration
- fix input calibration for -5 to 5V range
- send reset event when internal clock is stopped
- do not send reset when external clock is used
- prevent drift and stutter while changing bpm
- reset now actually reset to 0
- reduce clock out trigger length to 5ms
- rearrange the shift functions
- only conditionally run midi handler
- exponential and logarithmic curves were switched
- use exponential fader curve for global led brightness
- retain storage and parameters when app is moved
- fix sticky params race condition
- disable popover when dragging in layout
- properly check activeId against null
- fix clock transport commands
- limit extra reset sources
- fix subdivision numbers
- fix mute control
- fix LED on mute
- app not sending midi
- fix saving of the registers
- fix issues with sequences length is 16
- add about tab and attributions
- fix warping issues making it loose phase
- fix crash on unmute
- move color change to clock handler
- fix param size
- param change making the app crash
- apply correct pull for analog clock inputs
- properly parse enum defaultValue
- always pass through analog ticks
- reduce number of sample readings for adc
- increase codebook size for increased range
- add midi throttling to 500 messages per second
- disambiguate range names
- enable calibration data migration
- Change description
- add favicon
- add app params to manual
- lil update procedure fix
- fix routing for GitHub pages
- quick manual styling fixes
- fix app links to manual
- show device version in settings tab
- integrate manual into configurator
- add app parameters and storage to manual
- add troubleshooting link
- fix MIDI CC number offset
- fix bug preventing  going into free running mode
- fix CC output not reaching 127
- fix led feedback
- use debounced save for storage
- use proper bool default value
- actually fix the bug causing CC not reaching 127
- disable configurator version check
- add proper semver version check
- add package description for beta testing
- ensure gh-pages deployment pushes to correct branch
- change description for deployment testing
- fix some typo
- fix crash when pressing button 2 and moving fader 1
- make LED color more consistent
- clean up latch layer antipattern
- never erase calibration range
- add some variable safety
- fix some copy to reflect save/recall setup
- consider BASE_URL for icons
- fix recall of setup params
- properly redirect for firmware update
- support zero-velocity note-offs
- re-trigger gate on legato
- support zero-velocity note-offs
- remove filtering on MIDI CC
- add hardware factory reset
- add browser connection troubleshooting
- auto-inject firmware version from release-please manifest
- deploy configurator to versioned folders
- make landing page look like before
- try to autoconnect when coming from landing page
- redirect all hash links to the correct version
- support zero-velocity note-offs
- re-trigger gate on legato
- increase input gain in uni-polar mode
- turn LED button off when muted
- extend slew range, added passthrough a minimum
- more typo corrections
- cleanup scene recall
- increase dynamic range, simplify nomenclature
- prevent fader from freezing when clock is turned off in clocked mode
- correct inverted mute LED logic in scene handler
- remove filtering on MIDI CC
- set correct new length
- disable note off on stop when midi mode is not Note
- enable linting in CI
- fix range setting stuck on 0-10V
- set default source/destinations
- double usb MAX_PAYLOAD_SIZE to 512 bytes
- configurator link (#403)
- merge conflict artifact
- remove unused navigate parameter from connect function

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
