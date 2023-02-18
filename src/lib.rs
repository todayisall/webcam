use gloo_utils::format::JsValueSerdeExt;
use tracing::info;
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlVideoElement, MediaStreamConstraints, MediaStream};

pub struct VideoStream {
    el: HtmlVideoElement,
}

impl VideoStream {
    pub fn new(el: HtmlVideoElement) -> VideoStream {
        VideoStream { el: el }
    }

    pub async fn set_video_src(&self, video_constraints: &serde_json::Value) {
        let window = web_sys::window().unwrap();
        let navigator = window.navigator();
        let media_devices = navigator.media_devices().unwrap();
        info!("media_devices: {:?}", media_devices);

        let mut constraints = MediaStreamConstraints::new();

        constraints.video(&JsValue::from_serde(video_constraints).unwrap());
        constraints.audio(&false.into());

        let media = JsFuture::from(
            media_devices
                .get_user_media_with_constraints(&constraints)
                .unwrap(),
        )
        .await
        .unwrap();

        let media_stream = media.unchecked_into::<MediaStream>();
        info!("media_stream: {:?}", media_stream);

        self.el.set_src_object(Some(&media_stream));
    }
}
