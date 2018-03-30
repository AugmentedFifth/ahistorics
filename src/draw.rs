use drawable::Drawable;
use graphics;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use scene::Scene;
use settings::Settings;


pub const SPACING_FACTOR: f64 = 0.875;


/// Start drawing to the screen.
pub fn draw(gl_graphics: &mut GlGraphics,
            render_args: &RenderArgs,
            settings:    &Settings,
            scene:       &Scene)
{
    gl_graphics.draw(render_args.viewport(), |ctx, gl| {
        // Clear the entire window.
        graphics::clear(settings.colors.background_color, gl);

        // Draw the scene.
        scene.draw(&scene.camera, &ctx, gl);
    });
}
