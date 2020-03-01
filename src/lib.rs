use std::cell::RefCell;
use std::rc::Rc;

mod dom;
mod game;
mod input;
mod types;
mod utils;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();
    log("test");
    let (canvas, context) = dom::document::build_ui().unwrap();
    let mut canvas = dom::canvas::Canvas::new(canvas, context);
    let mut game = game::Game::new();
    game.tick();
    canvas.draw(&game);
    render_loop(canvas, game).unwrap();
}

fn render_loop(mut canvas: dom::canvas::Canvas, mut game: game::Game) -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        // requestAnimationFrame callback has fired.
        game.tick();
        canvas.draw(&game);
        // Schedule ourself for another requestAnimationFrame callback.
        dom::document::request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    dom::document::request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
