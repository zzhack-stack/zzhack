use crate::console_log;
use crate::Res;
use serde::de::DeserializeOwned;
use yew::format::Json;
use yew::Callback;

pub trait JSONCallback {
    fn wrap_callback<T>(callback: Callback<T>) -> Callback<Res<T>>
    where
        T: DeserializeOwned + 'static,
    {
        Callback::from(move |res: Res<T>| {
            let Json(body) = res.into_body();
            let data = match body {
                Ok(body) => body,
                Err(err) => {
                    console_log!("{}", err);
                    panic!("{}", err);
                }
            };

            callback.emit(data)
        })
    }
}
