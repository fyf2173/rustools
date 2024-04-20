use std::any::Any;
use std::sync::Arc;

use uuid::Uuid;

const TRACE_ID: &str = "trace_id";
const LOCALE: &str = "locale";
const LANG: &str = "lang";

pub enum Context {
    Wrap(Arc<Context>, String, Arc<dyn Any + Send + Sync>),
    Null,
}

pub fn new() -> Context {
    Context::Null
}

pub fn wrap(parent: Context) -> Context {
    if let Some(trace_id) = get_value::<String>(&parent, TRACE_ID.to_string()) {
        if trace_id != "" {
            return parent;
        }
    }
    let ctx = with_value(parent, TRACE_ID.to_string(), Uuid::new_v4().to_string());
    let ctx = with_value(ctx, LOCALE.to_string(), "zh");
    with_value(ctx, LANG.to_string(), "ZH-CN")
}

pub fn with_value<T: 'static + Send + Sync>(parent: Context, key: String, val: T) -> Context {
    Context::Wrap(Arc::new(parent), key, Arc::new(val))
}

pub fn get_value<T: 'static + Send + Sync>(mut cc: &Context, key: String) -> Option<&T> {
    loop {
        match cc {
            Context::Wrap(nextc, match_key, val) => {
                if key == match_key.as_ref() {
                    let tmp = val.downcast_ref::<T>()?;
                    return Some(tmp);
                }
                cc = nextc.as_ref();
            }
            Context::Null => break,
        };
    }
    None
}
