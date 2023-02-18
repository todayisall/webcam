use serde_json::json;
use sycamore::prelude::*;
use sycamore_futures::spawn_local_scoped;
use webcam::VideoStream;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    sycamore::render(|ctx| {
        view! { ctx,
            App()
        }
    });
}

#[component]
fn App<G: Html>(ctx: Scope) -> View<G> {
    view! { ctx,
        div(class="container") {
            Video()
        }
    }
}

// 视频组件
#[component]
fn Video<G: Html>(ctx: Scope) -> View<G> {
    let video_ref = create_node_ref(ctx);

    spawn_local_scoped(ctx, async move {
        let video = video_ref.get::<DomNode>().unchecked_into();
        let video_stream = VideoStream::new(video);
        video_stream
            .set_video_src(&json!({
                "audio": false,
                "video": {
                    "width": 1280,
                    "height": 720,
                    "frameRate": 30
                }
            }))
            .await;
    });

    view! { ctx,
        video(class="video", controls=true, autoplay=true, loop=true, muted=true, playsinline=true, preload="auto", ref=video_ref
    ) {
            "您的浏览器不支持 video 标签。"
        }
    }
}
