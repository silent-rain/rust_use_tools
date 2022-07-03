/*!日志-自定义日志输出
 * [dependencies]
 * tracing = "0.1.32"
 * tracing-subscriber = "0.3.9"
*/
use tracing::{debug_span, info, info_span};
use tracing_subscriber::prelude::*;

use seasnft7::tracing_logger::custom_layer::CustomLayer;

fn main() {
    tracing_subscriber::registry().with(CustomLayer).init();

    let outer_span = info_span!("outer", level = 0, other_field = tracing::field::Empty);
    let _outer_entered = outer_span.enter();
    // Some code...
    outer_span.record("other_field", &7);

    let inner_span = debug_span!("inner", level = 1);
    let _inner_entered = inner_span.enter();

    info!(a_bool = true, answer = 42, message = "first example");
}
