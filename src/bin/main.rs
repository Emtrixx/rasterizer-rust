use rasterizer::*;
use fermium::{
    prelude::*,
    video::*,
    *,
};
use glam::UVec2;

pub fn main() {
    let width = 1920;
    let height = 1080;


    unsafe {
        assert_eq!(SDL_Init(SDL_INIT_EVERYTHING), 0);
 
        let window = SDL_CreateWindow(
            b"demo\0".as_ptr().cast(),
            SDL_WINDOWPOS_CENTERED,
            SDL_WINDOWPOS_CENTERED,
            width,
            height,
            (SDL_WINDOW_OPENGL | SDL_WINDOW_ALLOW_HIGHDPI).0,
        );
        // Panic if window is not null
        assert!(!window.is_null());
 
        let renderer = SDL_CreateRenderer(window, -1, 1);
        // Panic if renderer is not null
        assert!(!renderer.is_null());
        SDL_SetRenderDrawColor(renderer, 0, 0, 255, 255);

        let mut event = SDL_Event::default();
        loop {
            assert_eq!(SDL_WaitEvent(&mut event), 1);
            match event.type_ {
              SDL_QUIT => {
                println!("SDL_QUIT");
                break;
              }
              _ => (),
            }//match
            // SDL_RenderClear(renderer);

            render(renderer, width as u32, height as u32);

            SDL_RenderPresent(renderer);

            SDL_Delay(10);
        }//loop
 
        SDL_DestroyWindow(window);
        SDL_Quit();
    }
}