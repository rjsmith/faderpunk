use embassy_futures::{
    select::{select, select3},
};
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, signal::Signal};
use heapless::Vec;
use libfp::{
    ext::FromValue,
    AppIcon, Brightness, Color, Config, Range, Param, Value, APP_MAX_PARAMS};

use serde::{Deserialize, Serialize};

use crate::app::{App, AppParams, AppStorage, Led, ManagedStorage, ParamStore, SceneEvent};

// TODO: Remove from final code
use defmt::info;

pub const CHANNELS: usize = 1;
pub const PARAMS: usize = 1;

// App configuration visible to the configurator
pub static CONFIG: Config<PARAMS> = Config::new(
    "My App",
    "Description of what this app does",
    Color::Blue,
    AppIcon::Fader,
)
.add_param(Param::Color {
    name: "Color",
    variants: &[
        Color::Blue,
        Color::Green,
        Color::Rose,
        Color::Orange,
        Color::Cyan,
        Color::Pink,
        Color::Violet,
        Color::Yellow,
    ],
});

pub struct Params {
    color: Color,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            color: Color::Blue,
        }
    }
}

impl AppParams for Params {
    fn from_values(values: &[Value]) -> Option<Self> {
        if values.len() < PARAMS {
            return None;
        }
        Some(Self {
            color: Color::from_value(values[0]),
        })
    }

    fn to_values(&self) -> Vec<Value, APP_MAX_PARAMS> {
        let mut vec = Vec::new();
        vec.push(self.color.into()).unwrap();
        vec
    }
}

// TODO: Make a macro to generate this.
#[derive(Serialize, Deserialize)]
pub struct Storage {
    fad_val: u16,
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            fad_val: 4095,
        }
    }
}

impl AppStorage for Storage {}

// Wrapper task - required for all apps
#[embassy_executor::task(pool_size = 16/CHANNELS)]
pub async fn wrapper(app: App<CHANNELS>, exit_signal: &'static Signal<NoopRawMutex, bool>) {
    let param_store = ParamStore::<Params>::new(app.app_id, app.layout_id);
    let storage = ManagedStorage::<Storage>::new(app.app_id, app.layout_id);

    param_store.load().await;
    storage.load().await;

      let app_loop = async {
        loop {
            select3(
                run(&app, &param_store, &storage),
                param_store.param_handler(),
                storage.saver_task(),
            )
            .await;
        }
    };

    select(app_loop, app.exit_handler(exit_signal)).await;
}

// Main app logic
pub async fn run(app: &App<CHANNELS>,
    params: &ParamStore<Params>,
    storage: &ManagedStorage<Storage>,
) {

    let (led_color,) = params
    .query(|p| {
        (
            p.color,
        )
    });

    let output = app.make_out_jack(0, Range::_0_10V).await;
    let fader = app.use_faders();
    let buttons = app.use_buttons();
    let leds = app.use_leds();

    leds.set(0, Led::Button, led_color, Brightness::Lower);

    let main_loop = async {
        buttons.wait_for_down(0).await;
        let value = fader.get_value();
        // TODO: Remove info! from final code
        info!("value {}", value);
        output.set_value(value);
        leds.set(0, Led::Top, led_color, Brightness::Custom((value / 16) as u8));
    };

    main_loop.await;

}