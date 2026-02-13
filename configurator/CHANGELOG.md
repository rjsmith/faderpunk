# Changelog

## 1.8.0 (2026-02-13)

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

## 1.8.0-beta.0 (2026-02-11)

### Features

- add jump and scale latch pickup modes

## 1.7.1 (2026-02-08)


### Bug Fixes

* remove unused navigate parameter from connect function ([73daa2a](https://github.com/ATOVproject/faderpunk/commit/73daa2a6706150434c5fbfcb7e88f0e818e32dcb))

## 1.7.0 (2026-02-08)


### Features

* **ad:** add MIDI retrigger param ([9586ddd](https://github.com/ATOVproject/faderpunk/commit/9586ddd1744a2f9c979d33b4b157aa40a37fe25f))
* add specialized midi params ([b288f57](https://github.com/ATOVproject/faderpunk/commit/b288f5720255efaf136464408599725ac0be9adf))
* **lfo_plus:** add lfo+ app ([70c0e08](https://github.com/ATOVproject/faderpunk/commit/70c0e08db9191077978210b2bdf3aabb5b56704a))
* **lfo+:** add reset cv destination ([#415](https://github.com/ATOVproject/faderpunk/issues/415)) ([999b274](https://github.com/ATOVproject/faderpunk/commit/999b2748e2dd3df546b598cd3d94e1e65df267d2))
* **midi2cv:** add velocity to gate param ([6cc69d0](https://github.com/ATOVproject/faderpunk/commit/6cc69d0b96ea2656e40b04266c9a5b510db5e5b2))
* **midi:** add documentation for v1.7 midi functions ([#416](https://github.com/ATOVproject/faderpunk/issues/416)) ([1dd560d](https://github.com/ATOVproject/faderpunk/commit/1dd560d0ab02e2641c838b147e2b2227df6cb62c))
* **midi:** midi out routing ([9d773b1](https://github.com/ATOVproject/faderpunk/commit/9d773b1806c5dbd52df542d4363190df5460756f))


### Bug Fixes

* **configurator:** enable linting in CI ([59c8de5](https://github.com/ATOVproject/faderpunk/commit/59c8de550a44facd3de89ac20955cbb66cc86f50))
* **led:** increase dynamic range, simplify nomenclature ([0e31ec7](https://github.com/ATOVproject/faderpunk/commit/0e31ec763b87c831eba9ef241cc8ba6850e71987))
* **manual:** configurator link ([#403](https://github.com/ATOVproject/faderpunk/issues/403)) ([d2779c6](https://github.com/ATOVproject/faderpunk/commit/d2779c656713263850fe4f247e6e3b06303667b8))
* **manual:** more typo corrections ([3209bfd](https://github.com/ATOVproject/faderpunk/commit/3209bfd035e2a50a936985298a729cd01d1278e7))

## 1.6.4 (2026-02-04)


### Bug Fixes

* **configurator:** make landing page look like before ([9404302](https://github.com/ATOVproject/faderpunk/commit/9404302907d999a0e8a729f44f43924306cc30ce))
* **configurator:** redirect all hash links to the correct version ([a6169ed](https://github.com/ATOVproject/faderpunk/commit/a6169ede4b2466f889c5aca0cb1a64cc8bdf5934))
* **configurator:** try to autoconnect when coming from landing page ([98b76d4](https://github.com/ATOVproject/faderpunk/commit/98b76d4e9afef4913d741e5a664e8f0f805b3662))

## 1.6.3 (2026-02-03)


### Bug Fixes

* **configurator:** deploy configurator to versioned folders ([437f5df](https://github.com/ATOVproject/faderpunk/commit/437f5df7909c895f7e41b0318b20dc0b6eedc2a1))

## 1.6.2 (2026-01-25)


### Bug Fixes

* add hardware factory reset ([e3fef02](https://github.com/ATOVproject/faderpunk/commit/e3fef022b1dbe497aed6c3875c32e3c481ca1297))
* **configurator:** add browser connection troubleshooting ([58ebc41](https://github.com/ATOVproject/faderpunk/commit/58ebc4159a38061d8460988dea654dcf277dabbb))
* **configurator:** auto-inject firmware version from release-please manifest ([a1adab9](https://github.com/ATOVproject/faderpunk/commit/a1adab94a3a345d6ee35c38055bf28099ec521d9))

## 1.6.1 (2025-12-08)


### Bug Fixes

* **configurator:** properly redirect for firmware update ([87e3dca](https://github.com/ATOVproject/faderpunk/commit/87e3dca452617e5296701fc86dab27eac9ff1003))

## 1.6.0 (2025-12-07)


### Features

* add beta release workflow for develop branch ([a2b53e5](https://github.com/ATOVproject/faderpunk/commit/a2b53e527aa9b3a8ad26e31ac545ffcb56c35ca9))
* add possibility to save and recall app layouts & params ([e724ac0](https://github.com/ATOVproject/faderpunk/commit/e724ac087893a3beb53ea4e4f39449dbb238ea68))
* bump minimum version to 1.5.0 ([fac1741](https://github.com/ATOVproject/faderpunk/commit/fac17412d1c5de3a39e96a868c9d4d54458fc7d7))
* **clock_div:** add clock divider app ([8214a38](https://github.com/ATOVproject/faderpunk/commit/8214a381734eb4eae5093b052888349c3a432995))
* **configurator:** add factory reset function ([85dc047](https://github.com/ATOVproject/faderpunk/commit/85dc047d3a8b463a746925df9de9c8778841c739))
* **configurator:** add Key and Tonic mapping to the manual ([feeecf0](https://github.com/ATOVproject/faderpunk/commit/feeecf0016221613179044eb81c7932c2f2e023a))
* **configurator:** store config with layout in setup file ([0d4fd4e](https://github.com/ATOVproject/faderpunk/commit/0d4fd4e4bc4791298987e24f4747662a824a0d8c))
* **manual:** update Turing and Turing+ manual ([a1d8c46](https://github.com/ATOVproject/faderpunk/commit/a1d8c46e57b97d284cf7d8bc5a9bc542acf13a20))
* **midi2cv:** add velocity to `Gate` and `Note Gate` mode ([0d1af52](https://github.com/ATOVproject/faderpunk/commit/0d1af527f043444cdca86e41e00f9963fcdd0339))
* **panner:** add panner app ([d990c75](https://github.com/ATOVproject/faderpunk/commit/d990c752e5bdd51704a1323dfc798c50b7ba33e6))


### Bug Fixes

* **configurator:** consider BASE_URL for icons ([55ee6ab](https://github.com/ATOVproject/faderpunk/commit/55ee6ab32aebf80b36a8d7deaf5ab75384f794a8))
* **configurator:** fix recall of setup params ([0252d77](https://github.com/ATOVproject/faderpunk/commit/0252d77477f33db10f1f4892456ba36381ef4425))
* **configurator:** fix some copy to reflect save/recall setup ([f9c57ea](https://github.com/ATOVproject/faderpunk/commit/f9c57ea7a64198a8d8c79ffc31b666fd7821bb6d))
* **manual:** fix some typo ([e60559b](https://github.com/ATOVproject/faderpunk/commit/e60559b2d4f5d8340c32970aa2e97fb19135f43f))

## 1.5.4 (2025-11-08)


### Bug Fixes

* **configurator:** change description for deployment testing ([de51654](https://github.com/ATOVproject/faderpunk/commit/de51654d8173a59e2045abe8515c66ce270fc26d))

## 1.5.3 (2025-11-08)


### Bug Fixes

* **configurator:** add package description for beta testing ([0049628](https://github.com/ATOVproject/faderpunk/commit/0049628dd29c81bd76c4d90b02994de37bc52a26))

## 1.5.2 (2025-10-27)


### Bug Fixes

* **configurator:** add proper semver version check ([2d19880](https://github.com/ATOVproject/faderpunk/commit/2d19880aa5bc3b1ed0b55ea80fbd08fbf511c15a))

## 1.5.1 (2025-10-25)


### Bug Fixes

* **configurator:** disable configurator version check ([284994f](https://github.com/ATOVproject/faderpunk/commit/284994f1b60a743d03e1ddae28b4c40121c4c95f))

## 1.5.0 (2025-10-24)


### Features

* **manual:** update app manual for v1.4.0 ([aa4e1a8](https://github.com/ATOVproject/faderpunk/commit/aa4e1a8eb90faa8210cea510e0c37056666d29af))


### Bug Fixes

* **configurator:** use proper bool default value ([3655941](https://github.com/ATOVproject/faderpunk/commit/3655941bfef41835c5396c851e45e447f28b4030))

## 1.4.0 (2025-10-21)


### Features

* **configurator:** add button to clear apps ([0c0a009](https://github.com/ATOVproject/faderpunk/commit/0c0a009cf2f81465bafa899d1dc1f46d7f170aac))

## 1.3.2 (2025-10-15)


### Bug Fixes

* **configurator:** add troubleshooting link ([f8595a8](https://github.com/ATOVproject/faderpunk/commit/f8595a8c8c6eb68503c874d82b0fd2ccdcb51484))

## 1.3.1 (2025-10-14)


### Bug Fixes

* **manual:** add app parameters and storage to manual ([8d0e976](https://github.com/ATOVproject/faderpunk/commit/8d0e9764102711f85be2cd3df40cc2a23ed6e1cf))

## 1.3.0 (2025-10-10)


### Features

* **configurator:** add mvm ([6b5da0d](https://github.com/ATOVproject/faderpunk/commit/6b5da0dda5050dd381d745d9fe097c867a5eb4cd))


### Bug Fixes

* **configurator:** integrate manual into configurator ([421e8e0](https://github.com/ATOVproject/faderpunk/commit/421e8e0a20e9b916d4a7ee06aecf4bebfee9d224))
* **configurator:** show device version in settings tab ([f23d99a](https://github.com/ATOVproject/faderpunk/commit/f23d99ae27ab8d00903d625c0c01467e4f5bebf6))

## 1.2.3 (2025-10-10)


### Bug Fixes

* **configurator:** fix app links to manual ([d2667ba](https://github.com/ATOVproject/faderpunk/commit/d2667ba2be17f8ad061c3a959d8a2fe4981639bc))
* **configurator:** quick manual styling fixes ([f5064e7](https://github.com/ATOVproject/faderpunk/commit/f5064e79722c6b8c5b33f75d9a5ae550a976904d))

## 1.2.2 (2025-10-09)


### Bug Fixes

* **configurator:** fix routing for GitHub pages ([492e4e0](https://github.com/ATOVproject/faderpunk/commit/492e4e0fa7e15e6c9a06de207a7c1fafb273ea7b))

## 1.2.1 (2025-10-09)


### Bug Fixes

* **configurator:** lil update procedure fix ([57a4be6](https://github.com/ATOVproject/faderpunk/commit/57a4be60a0d82dd4e61fb84e94aeb927f3bdfc94))

## 1.2.0 (2025-10-09)


### Features

* **configurator:** add initial version of all app manuals ([97b56ac](https://github.com/ATOVproject/faderpunk/commit/97b56ac8cc710de155e91d75991c91e817336086))
* **configurator:** add manual page ([85385e0](https://github.com/ATOVproject/faderpunk/commit/85385e0b8e120beb1da348b64f72b74a90878daf))
* **configurator:** add manual template ([7085070](https://github.com/ATOVproject/faderpunk/commit/7085070a91ee656017a0609ef877c792688835ba))
* **configurator:** add update guide and fw link ([d7107ef](https://github.com/ATOVproject/faderpunk/commit/d7107ef0bc6f471b7db07487f801f2b8fd98ba2f))
* **configurator:** display update message ([7d9894f](https://github.com/ATOVproject/faderpunk/commit/7d9894f1a00586aadf572688de869befd9213318))
* **configurator:** manual app style improvements ([e49bde3](https://github.com/ATOVproject/faderpunk/commit/e49bde3ae6fc3673980b923e082d92b676ba0104))


### Bug Fixes

* **configurator:** add app params to manual ([155e9dd](https://github.com/ATOVproject/faderpunk/commit/155e9dd28b5538a9c38c537d81168a478c6c8c3e))
* **configurator:** add favicon ([86f45d4](https://github.com/ATOVproject/faderpunk/commit/86f45d4a64771430ea2af9c43db3ed65ae9378e0))

## 1.1.1 (2025-10-08)


### Bug Fixes

* **configurator:** disambiguate range names ([87947ef](https://github.com/ATOVproject/faderpunk/commit/87947eff463d2df42dd188c7e4e625f18bbcfc08))
* **configurator:** properly parse enum defaultValue ([00db2a0](https://github.com/ATOVproject/faderpunk/commit/00db2a0a3bf569ce80076519ba075b3a451232b6))

## 1.1.0 (2025-09-25)


### Features

* **configurator:** rename params, fix float field ([9349aa6](https://github.com/ATOVproject/faderpunk/commit/9349aa624432e3aef66b71a7a1a19e2b40dacef8))


### Bug Fixes

* **clock:** limit extra reset sources ([7fc8619](https://github.com/ATOVproject/faderpunk/commit/7fc861910648376d5f7963214c1c6f2a33df7bd5))
* **configurator:** add about tab and attributions ([8d9ab89](https://github.com/ATOVproject/faderpunk/commit/8d9ab8931922e0896094a5cd518bd5de71b207ca))

## 1.0.0 (2025-09-20)


### ⚠ BREAKING CHANGES

* **configurator:** release configurator 1.0

### Features

* **configurator:** connect page, minor additions ([1cdc8fa](https://github.com/ATOVproject/faderpunk/commit/1cdc8fa2aa7c5317e34098bbccf467846a3ef4a7))
* **configurator:** release configurator 1.0 ([92e3091](https://github.com/ATOVproject/faderpunk/commit/92e30914e5ff6fb1166a851732133617dbcc89ac))
* **configurator:** remove old configurator ([b7a6e8d](https://github.com/ATOVproject/faderpunk/commit/b7a6e8dbf9178e843c263c4dd770563a45285b53))
* **configurator:** save global settings ([f4327d5](https://github.com/ATOVproject/faderpunk/commit/f4327d5cf02dc863f2a128905cf3f416ac6e40ce))


### Bug Fixes

* **configurator:** disable popover when dragging in layout ([63dc2ba](https://github.com/ATOVproject/faderpunk/commit/63dc2bae4d2ace8bd0af23505d5678ba0ef9c79e))
* **configurator:** properly check activeId against null ([90fb701](https://github.com/ATOVproject/faderpunk/commit/90fb701aa63a5194b88faac822afe6193f6b051a))

## 0.2.1 (2025-08-21)


### Bug Fixes

* **configurator:** fix color param not being sent ([9ba5bc9](https://github.com/ATOVproject/faderpunk/commit/9ba5bc90c3f8f7cfe6ddf721e7f45ae085234d3e))

## 0.2.0 (2025-08-21)


### Features

* add and set params for apps ([55317b9](https://github.com/ATOVproject/faderpunk/commit/55317b90ed6b0cb6c315737603fbe55b6cc37220))
* add HeroUI based suuuuper basic configurator ([6c9d8f8](https://github.com/ATOVproject/faderpunk/commit/6c9d8f883761ea245638a462122535bff55e4091))
* add postcard encoded app config list ([e8889cd](https://github.com/ATOVproject/faderpunk/commit/e8889cdf681f7d432e7dd9eb648a76410ab0928d))
* **config:** retrieve app state from configurator ([1b9d105](https://github.com/ATOVproject/faderpunk/commit/1b9d10513b0fccf923d367e88b76872f50467938))
* **config:** separate layout from global config ([54d8690](https://github.com/ATOVproject/faderpunk/commit/54d869014c2299812519a4b47cc0b8a9a069a09f))
* **config:** set a param from configurator ([de47407](https://github.com/ATOVproject/faderpunk/commit/de47407a0ea913dcefe5767019b7a988b2661d00))
* **configurator:** deploy to Github pages ([a84aa5f](https://github.com/ATOVproject/faderpunk/commit/a84aa5f0d548b33d78e2722e2de2ae2b764ae791))
* **configurator:** implement layout setting ([17cb7b3](https://github.com/ATOVproject/faderpunk/commit/17cb7b338c8764302ada0ed4b54e7c74fbd5e2db))
* **configurator:** set custom layouts ([8902af6](https://github.com/ATOVproject/faderpunk/commit/8902af6f3f433e0046f3a445e4d1d1ed91483a10))
* decode large configuration messages ([e415f13](https://github.com/ATOVproject/faderpunk/commit/e415f13e740f2ac7efae0b40bdc85e65598376de))
* implement dynamic scene changes ([0a12ed6](https://github.com/ATOVproject/faderpunk/commit/0a12ed65d04c60a72a0a9dc9b218d6b34c605894))
* make max and midi channels CriticalSectionRawMutex Channels ([e0617e5](https://github.com/ATOVproject/faderpunk/commit/e0617e556b9a887034b695d6cd118cb8672d4d64))
* move param handler into param store ([27aee71](https://github.com/ATOVproject/faderpunk/commit/27aee71d40f784e74e65201195e7d071e3d9fca0))
* **params:** add Color param component in configurator ([8428a20](https://github.com/ATOVproject/faderpunk/commit/8428a2069de88721c4c2373792bc46f95794d57b))
* set clock sources using the configurator ([08f5312](https://github.com/ATOVproject/faderpunk/commit/08f53126e9e02a33855cb07861ad49d1c4b3c8cc))
* show params in configurator temp page ([99f8e69](https://github.com/ATOVproject/faderpunk/commit/99f8e696ff35a273907058d69d09a4ed2c1d87f2))
* **usb:** establish basic webusb connection ([6f3a418](https://github.com/ATOVproject/faderpunk/commit/6f3a4183bc3ab75ac49c3c28462d2f952a51ceee))
* **usb:** fix webusb windows compatibility ([fb01f98](https://github.com/ATOVproject/faderpunk/commit/fb01f981c64beb133b50f6072ae73fe30f113e3b))
* use batch messages for app listing ([da76ce1](https://github.com/ATOVproject/faderpunk/commit/da76ce1f72f577b91a74a1f3b4c101f88b33cfa9))


### Bug Fixes

* **configurator:** adjust for i2c global params ([0378d9b](https://github.com/ATOVproject/faderpunk/commit/0378d9b49e18e37b0179a113acb33ce53192f07d))
* **configurator:** adjust transformValues for 8 params ([34c7e08](https://github.com/ATOVproject/faderpunk/commit/34c7e0865c7476c1535dd17d778e71f093751869))
* **configurator:** check in pnpm-lock.yaml ([cc564fd](https://github.com/ATOVproject/faderpunk/commit/cc564fdc36461a7c818a7364ec19adf0e5bd2a64))
* restructure GlobalConfig to be Serialize, Deserialize ([b69c2ff](https://github.com/ATOVproject/faderpunk/commit/b69c2ff00d051807032c862c7e4320439dbb04e5))
