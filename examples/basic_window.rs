// This example is ported from the official raylib basic window example.
// https://www.raylib.com/examples/core/loader.html?name=core_basic_window

use rraylib::sys::*;
use rraylib::{LIGHTGRAY, RAYWHITE};

//------------------------------------------------------------------------------------
// Program main entry point
//------------------------------------------------------------------------------------
fn main() {
    const SCREEN_WIDTH: i32 = 800;
    const SCREEN_HEIGHT: i32 = 450;

    unsafe {
        // Initialization
        //--------------------------------------------------------------------------------------
        InitWindow(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            b"raylib [core] example - basic window\0".as_ptr() as *const i8,
        );
        SetTargetFPS(60);
        //--------------------------------------------------------------------------------------

        // Main game loop
        while !WindowShouldClose()
        // Detect window close button or ESC key
        {
            // Update
            //----------------------------------------------------------------------------------
            // TODO: Update your variables here
            //----------------------------------------------------------------------------------

            // Draw
            //----------------------------------------------------------------------------------
            BeginDrawing();

            ClearBackground(RAYWHITE);

            DrawText(
                b"Congrats! You created your first window!\0".as_ptr() as *const i8,
                190,
                200,
                20,
                LIGHTGRAY,
            );

            EndDrawing();
            //----------------------------------------------------------------------------------
        }

        // De-Initialization
        //--------------------------------------------------------------------------------------
        CloseWindow(); // Close window and OpenGL context

        //-----------------------------------------------------------------------
    }
}
