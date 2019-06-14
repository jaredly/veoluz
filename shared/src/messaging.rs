use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: usize,
    pub config: crate::Config,
    pub count: usize,
}

// enum MessageForWorker {
//   Render { id: usize, config: crate::Config }
// }

// enum MessageForMain {
//   Rendered { id: usize, data: js_sys::Uint32Array }
// }
