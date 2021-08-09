use photon_rs::{filters::filter, PhotonImage};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::html::ChangeData;
use yew::prelude::{
    html, App as YewApp, Callback, Component, ComponentLink, Html, NodeRef, ShouldRender,
};
use yew::services::{
    reader::{FileData, ReaderTask},
    ConsoleService, ReaderService,
};
use yew::web_sys::{
    CanvasRenderingContext2d, File, FileReader, HtmlCanvasElement, HtmlInputElement, Url,
};

struct App {
    link: ComponentLink<Self>,
    input_ref: NodeRef,
    canvas_ref: NodeRef,
    reader_task: Option<ReaderTask>,
}

enum AppMsg {
    SelectFile(ChangeData),
    LoadFile(FileData),
    FlipImage,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            input_ref: NodeRef::default(),
            canvas_ref: NodeRef::default(),
            reader_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMsg::SelectFile(change_data) => {
                ConsoleService::info("file selected");
                if let ChangeData::Files(files) = change_data {
                    let file = files.item(0).unwrap();
                    let callback = self.link.callback(|file_data| AppMsg::LoadFile(file_data));
                    self.reader_task = Some(ReaderService::read_file(file, callback).unwrap());
                }
                false
            }
            AppMsg::LoadFile(file_data) => {
                ConsoleService::info("file loaded");
                ConsoleService::info(format!("{:?}", file_data.content.clone()).as_str());
                let mut img = PhotonImage::new_from_byteslice(file_data.content);
                ConsoleService::info("photon image created");
                ConsoleService::info(format!("{:?}", img).as_str());
                let canvas = self
                    .canvas_ref
                    .cast::<HtmlCanvasElement>()
                    .expect("failed to cast canvas");
                let ctx = canvas
                    .get_context("2d")
                    .expect("failed to get context")
                    .expect("failed to get context")
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();
                canvas.set_height(img.get_height());
                canvas.set_width(img.get_width());
                filter(&mut img, "twenties");
                photon_rs::putImageData(canvas, ctx, img);
                ConsoleService::info("photon image put");
                true
            }
            AppMsg::FlipImage => {
                let canvas = self
                    .canvas_ref
                    .cast::<HtmlCanvasElement>()
                    .expect("failed to cast canvas");
                let ctx = canvas
                    .get_context("2d")
                    .expect("failed to get context")
                    .expect("failed to get context")
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();
                let mut img = photon_rs::open_image(canvas.clone(), ctx.clone());
                photon_rs::transform::fliph(&mut img);
                photon_rs::putImageData(canvas, ctx, img);
                ConsoleService::info("photon image put");
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div>
                    <input type="file" ref={self.input_ref.clone()} id="image" name="example" accept="image/jpeg, image/png" onchange=self.link.callback(|change_data| AppMsg::SelectFile(change_data))/>
                </div>
                <div>
                    <button onclick={self.link.callback(|_| AppMsg::FlipImage)}>{ "Flip" }</button>
                </div>
                <div>
                    <canvas ref={self.canvas_ref.clone()} id="canvas" width="60" height="40"></canvas>
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
