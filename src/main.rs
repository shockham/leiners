extern crate caper;
extern crate time;

use caper::game::*;
use caper::imgui::Ui;
use caper::input::Key;
use caper::mesh::gen_sphere;
use caper::posteffect::PostShaderOptionsBuilder;
use caper::types::{DefaultTag, MaterialBuilder, RenderItemBuilder, TransformBuilder};

use std::f32::consts::PI;

mod shaders;

fn main() {
    let mut game = Game::<DefaultTag>::new();
    let mut debug_mode = false;

    // generate the instance positions
    let transforms = (0..200)
        .map(|i| {
            TransformBuilder::default()
                .pos(((i as f32 / 10f32) * 2f32, (i as f32 % 10f32) * 2f32, 0f32))
                .rot((0f32, 0f32, 0f32, 1f32))
                .scale((1f32, 1f32, 1f32))
                .build()
                .unwrap()
        }).collect::<Vec<_>>();

    // create a vector of render items
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_sphere())
            .material(
                MaterialBuilder::default()
                    .shader_name("contours")
                    .build()
                    .unwrap(),
            ).instance_transforms(transforms)
            .build()
            .unwrap(),
    );

    // example of how to configure the default post effect shader
    game.renderer.post_effect.post_shader_options = PostShaderOptionsBuilder::default()
        .chrom_amt(1f32)
        .blur_amt(2f32)
        .blur_radius(6f32)
        .bokeh(true)
        .bokeh_focal_depth(0.45f32)
        .bokeh_focal_width(0.4f32)
        .color_offset((1f32, 0.8f32, 1f32, 1f32))
        .noise(0.3f32)
        .scanline(0.04f32)
        .scanline_count(200i32)
        .build()
        .unwrap();

    // initial setup
    {
        shaders::add_custom_shaders(&mut game);
        game.cams[0].pos = (20f32, 10f32, 25f32);
    }

    loop {
        // run the engine update
        let status = game.update(
            |_: &Ui| {},
            |g: &mut Game<DefaultTag>| -> UpdateStatus {
                // update some items
                let update_time = time::precise_time_s();

                // update the first person inputs
                //movement::handle_inputs(&mut g.input, &mut g.cams[0]);
                g.cams[0].pos = (
                    20f32 + update_time.sin() as f32 * 5f32,
                    10f32,
                    25f32 + update_time.cos() as f32 * 5f32,
                );

                g.cams[0].euler_rot = (
                    0f32,
                    (PI / 16f32) * update_time.sin() as f32,
                    0f32,
                );

                let (scale_mul_x, scale_mul_y) = if g.input.keys_down.contains(&Key::S) {
                    (5f32, 5f32)
                } else if g.input.keys_down.contains(&Key::A) {
                    (5f32, 1f32)
                } else if g.input.keys_down.contains(&Key::D) {
                    (1f32, 5f32)
                } else {
                    (1f32, 1f32)
                };

                {
                    g.get_render_item(0).material.shader_name = if g.input.keys_down.contains(&Key::W){
                        "contours_col".to_string()
                    } else {
                        "contours".to_string()
                    };
                }

                g.renderer.post_effect.post_shader_options.blur_radius = if g.input.keys_down.contains(&Key::E) {
                    3f32
                } else {
                    6f32 
                };
                
                g.renderer.post_effect.post_shader_options.scanline = if g.input.keys_down.contains(&Key::Q) {
                    0f32
                } else {
                    0.04f32 
                };


                for t in g.get_render_item(0).instance_transforms.iter_mut() {
                    t.pos = (
                        t.pos.0,
                        t.pos.1,
                        ((t.pos.0 / 5f32).sin()
                            * (t.pos.1 / 5f32).cos()
                            * update_time.sin() as f32)
                            * 2f32,
                    );

                    let scale = (update_time as f32 + (t.pos.0 + t.pos.1) / 5f32)
                        .sin()
                        .abs();
                    t.scale = (
                        scale * scale_mul_x,
                        scale * scale_mul_y,
                        scale * update_time.tan() as f32
                    );
                }

                // gif
                if g.input.keys_pressed.contains(&Key::O) {
                    g.renderer.save_add_to_gif("test.gif");
                }

                // screenshot
                if g.input.keys_pressed.contains(&Key::P) {
                    g.renderer.save_screenshot();
                }

                // show the debugger
                if g.input.keys_down.contains(&Key::LShift) {
                    if g.input.keys_down.contains(&Key::L) {
                        debug_mode = true;
                    }
                    if g.input.keys_down.contains(&Key::K) {
                        debug_mode = false;
                    }
                    g.input.hide_mouse = !g.input.keys_down.contains(&Key::M);
                }
                g.renderer.show_editor = debug_mode;

                // quit
                if g.input.keys_down.contains(&Key::Escape) {
                    return UpdateStatus::Finish;
                }

                UpdateStatus::Continue
            },
        );

        if let UpdateStatus::Finish = status {
            break;
        }
    }
}
