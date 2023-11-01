use bevy::prelude::*;
pub trait GameFonts {
    fn font_future(&self) -> Handle<Font>;
    fn font_future_thin(&self) -> Handle<Font>;
}

impl GameFonts for AssetServer {
    fn font_future(&self) -> Handle<Font> {
        self.load("fonts/kenvector_future.ttf")
    }

    fn font_future_thin(&self) -> Handle<Font> {
        self.load("fonts/kenvector_future_thin.ttf")
    }
}
