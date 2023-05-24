//
// Debug related console output functions
//
use once_cell::sync::Lazy;              // Needed for initializing APP_START
use std::time::{Instant, SystemTime};

// Formatting time is HARD :/
use time::format_description::FormatItem;
use time::macros::format_description;
// use time::format_description::well_known::Iso8601;
const SHORT_ISO8601: &'static [FormatItem<'static>] = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");


// Gets initialized when USED first time :/
static APP_START: Lazy<Instant> = Lazy::new(Instant::now);

// NOTE:
// Because this `debug` SUBMODULE is defined as private in the PARENT module entry point (`mod.rs`),
// using `pub` makes these functions visible ONLY inside the PARENT and SIBLING modules, EXCEPT if
// made visible outside by exporting them further using `pub use debug::{...}` or prelude definition.

pub fn print_running_time() {
    let now: time::OffsetDateTime = SystemTime::now().into();
    let dt = now.format(&SHORT_ISO8601).unwrap();
    // let dt = now.format(&Iso8601::DEFAULT).expect("Failed to format given date using ISO8601");

    println!("{} :: DEBUG :: Total running time: {:?}", dt, APP_START.elapsed());
}
