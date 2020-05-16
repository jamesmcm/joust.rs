use super::super::game::Game;
use super::super::utils::nop;
use super::document;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const BACKGROUND_COLOR: &'static str = "#000000";

pub struct Canvas {
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
}

impl Canvas {
    pub fn new(
        canvas: web_sys::HtmlCanvasElement,
        context: web_sys::CanvasRenderingContext2d,
    ) -> Canvas {
        Canvas { canvas, context }
    }
    pub fn draw(&mut self, game: &Game) -> () {
        self.context.begin_path();

        // Fill canvas
        self.context
            .set_fill_style(&wasm_bindgen::JsValue::from_str(BACKGROUND_COLOR));
        self.context.fill_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );

        self.context
            .set_fill_style(&wasm_bindgen::JsValue::from_str("#FFFFFF"));
        for platform in game.platforms.iter() {
            self.context.fill_rect(
                platform.position.x,
                self.canvas.height() as f64 - platform.width as f64 - platform.position.y,
                platform.length,
                platform.width,
            );
        }

        self.context
            .set_fill_style(&wasm_bindgen::JsValue::from_str("#FF0000"));
        for enemy in game.other_jousters.iter() {
            self.context.fill_rect(
                enemy.position.x,
                self.canvas.height() as f64 - enemy.height as f64 - enemy.position.y,
                enemy.width as f64,
                enemy.height as f64,
            );
        }

        // Draw player
        self.context
            .set_fill_style(&wasm_bindgen::JsValue::from_str("#FFFFFF"));
        self.context.fill_rect(
            game.player.jouster.position.x,
            self.canvas.height() as f64
                - game.player.jouster.height as f64
                - game.player.jouster.position.y,
            game.player.jouster.width as f64,
            game.player.jouster.height as f64,
        );
        // Draw others

        self.context.stroke();
        let nop_closure = Closure::new(nop);
        let _animation_frame = document::request_animation_frame(&nop_closure);
        nop_closure.forget();
    }
}
