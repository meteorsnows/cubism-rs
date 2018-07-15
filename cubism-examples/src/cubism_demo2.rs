extern crate cubism;
extern crate cubism_gfx_renderer;
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate nalgebra as na;
#[macro_use]
extern crate imgui;
extern crate imgui_gfx_renderer;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

use cubism::Model;
use gfx::Device;
use gfx::Factory;
use glutin::Api::OpenGl;
use glutin::{GlContext, GlRequest};
use image::GenericImage;
use imgui::ImGuiMouseCursor;
use imgui::ImGui;
use imgui::{ImStr, ImString};
use std::time::Instant;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (i32, i32),
    pressed: (bool, bool, bool),
    wheel: f32,
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new().with_gl(GlRequest::Specific(OpenGl, (3, 2)));
    let (window, mut device, mut factory, mut rtv, mut stv) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_builder, context, &events_loop);

    let mut imgui = imgui::ImGui::init();
    {
        // Fix incorrect colors with sRGB framebuffer
        use imgui::ImVec4;

        fn imgui_gamma_to_linear(col: ImVec4) -> ImVec4 {
            let x = col.x.powf(2.2);
            let y = col.y.powf(2.2);
            let z = col.z.powf(2.2);
            let w = 1.0 - (1.0 - col.w).powf(2.2);
            ImVec4::new(x, y, z, w)
        }

        let style = imgui.style_mut();
        for col in 0..style.colors.len() {
            style.colors[col] = imgui_gamma_to_linear(style.colors[col]);
        }
    }
    imgui.set_ini_filename(None);
    let mut imrenderer = imgui_gfx_renderer::Renderer::init(&mut imgui, &mut factory, imgui_gfx_renderer::Shaders::GlSlEs300, rtv.clone())
        .expect("Failed to initialize renderer");

    let mut model = Model::from_bytes(include_bytes!("../res/Koharu.moc3")).unwrap();
    let im = image::open("cubism-examples/res/Koharu.png")
        .unwrap()
        .flipv();
    let (_, texture) = factory
        .create_texture_immutable_u8::<gfx::format::Rgba8>(
            gfx::texture::Kind::D2(
                im.width() as u16,
                im.height() as u16,
                gfx::texture::AaMode::Single,
            ),
            gfx::texture::Mipmap::Provided,
            &[&im.raw_pixels()],
        )
        .unwrap();
    let sampler = factory.create_sampler(gfx::texture::SamplerInfo::new(
        gfx::texture::FilterMethod::Scale,
        gfx::texture::WrapMode::Clamp,
    ));

    let mut model_renderer = cubism_gfx_renderer::Renderer::init(&mut factory, rtv.clone()).unwrap();

    let mut last_frame = Instant::now();
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let mut running = true;
    let mut mouse_state = MouseState::default();

    let parameter_names = {
        let mut vec = Vec::with_capacity(model.parameter_count());
        for idx in 0..model.parameter_count() {
            vec.push(unsafe { ImString::from_utf8_with_nul_unchecked([model.parameter_ids()[idx], "\0"].concat().into_bytes()) })
        }
        vec
    };
    let part_names = {
        let mut vec = Vec::with_capacity(model.part_count());
        for idx in 0..model.part_count() {
            vec.push(unsafe { ImString::from_utf8_with_nul_unchecked([model.part_ids()[idx], "\0"].concat().into_bytes()) })
        }
        vec
    };
    let drawable_names = {
        let mut vec = Vec::with_capacity(model.drawable_count());
        for idx in 0..model.drawable_count() {
            vec.push(unsafe { ImString::from_utf8_with_nul_unchecked([model.drawable_ids()[idx], "\0"].concat().into_bytes()) })
        }
        vec
    };

    while running {
        events_loop.poll_events(|event| {
            use glutin::WindowEvent::*;
            use glutin::ElementState::Pressed;
            use glutin::{Event, MouseButton, MouseScrollDelta, TouchPhase};

            if let Event::WindowEvent { event, .. } = event {
                match event {
                    Resized(w, h) => {
                        gfx_window_glutin::update_views(&window, &mut rtv, &mut stv);
                        imrenderer.update_render_target(rtv.clone());
                    }
                    CloseRequested => running = false,
                    KeyboardInput { input, .. } => {
                        use glutin::VirtualKeyCode as Key;

                        let pressed = input.state == Pressed;
                        if let Some(vk) = input.virtual_keycode {
                            update_imgui_keys(&mut imgui, vk, pressed);
                        }
                    }
                    CursorMoved { position: (x, y), .. } => mouse_state.pos = (x as i32, y as i32),
                    MouseInput { state, button, .. } => {
                        match button {
                            MouseButton::Left => mouse_state.pressed.0 = state == Pressed,
                            MouseButton::Right => mouse_state.pressed.1 = state == Pressed,
                            MouseButton::Middle => mouse_state.pressed.2 = state == Pressed,
                            _ => {}
                        }
                    }
                    MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_, y),
                        phase: TouchPhase::Moved,
                        ..
                    } |
                    MouseWheel {
                        delta: MouseScrollDelta::PixelDelta(_, y),
                        phase: TouchPhase::Moved,
                        ..
                    } => mouse_state.wheel = y,
                    ReceivedCharacter(c) => imgui.add_input_character(c),
                    _ => (),
                }
            }
        });

        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;

        update_mouse(&mut imgui, &mut mouse_state);


        let mouse_cursor = imgui.mouse_cursor();

        window.set_cursor(match mouse_cursor {
            ImGuiMouseCursor::None => unreachable!("mouse_cursor was None!"),
            ImGuiMouseCursor::Arrow => glutin::MouseCursor::Arrow,
            ImGuiMouseCursor::TextInput => glutin::MouseCursor::Text,
            ImGuiMouseCursor::Move => glutin::MouseCursor::Move,
            ImGuiMouseCursor::ResizeNS => glutin::MouseCursor::NsResize,
            ImGuiMouseCursor::ResizeEW => glutin::MouseCursor::EwResize,
            ImGuiMouseCursor::ResizeNESW => glutin::MouseCursor::NeswResize,
            ImGuiMouseCursor::ResizeNWSE => glutin::MouseCursor::NwseResize,
        });

        let size_pixels = window.get_inner_size().unwrap();
        let hdipi = window.hidpi_factor();
        let size_points = (
            (size_pixels.0 as f32 / hdipi) as u32,
            (size_pixels.1 as f32 / hdipi) as u32,
        );

        let ui = imgui.frame(size_points, size_pixels, delta_s);
        ui.main_menu_bar(|| {
            ui.label_text(im_str!("Delta: {}", delta_s), im_str!(""));
        });
        ui.window(im_str!("CharParams"))
            .size((300.0, 100.0), imgui::ImGuiCond::FirstUseEver)
            .build(|| {
                for idx in 0..model.parameter_count() {
                    let min = model.parameter_min()[idx];
                    let max = model.parameter_max()[idx];
                    ui.slider_float(
                        &parameter_names[idx],
                        &mut model.parameter_values_mut()[idx],
                        min,
                        max
                    ).build();
                }
            });
        ui.window(im_str!("CharParts"))
            .size((300.0, 100.0), imgui::ImGuiCond::FirstUseEver)
            .build(|| {
                for idx in 0..model.part_count() {
                    ui.slider_float(
                        &part_names[idx],
                        &mut model.part_opacities_mut()[idx],
                        0.0,
                        1.0
                    ).build();
                }
            });
        /*
        ui.window(im_str!("CharDrawables"))
            .size((300.0, 100.0), imgui::ImGuiCond::FirstUseEver)
            .build(|| {
                for idx in 0..model.drawable_count() {
                    ui.label_text(
                        im_str!(""),
                        &drawable_names[idx]
                    );
                }
            });*/
        model.update();

        encoder.clear(&rtv, [0.0, 0.2, 0.0, 1.0]);

        model_renderer.set_mvp(na::Matrix4::<f32>::identity());
        model_renderer
            .draw_model(&mut factory, &mut encoder, &model, (&texture, &sampler))
            .unwrap();
        model_renderer.set_mvp(na::Matrix4::new_scaling(2.0));

        imrenderer.render(ui, &mut factory, &mut encoder).expect(
            "Rendering failed",
        );
        encoder.flush(&mut device);

        window.swap_buffers().unwrap();
        device.cleanup();
        //::std::thread::sleep_ms(16);
    }
}

#[inline]
fn update_imgui_keys(imgui: &mut ImGui, vk: glutin::VirtualKeyCode, pressed: bool) {
    use glutin::VirtualKeyCode as Key;
    match vk {
        Key::Tab => imgui.set_key(0, pressed),
        Key::Left => imgui.set_key(1, pressed),
        Key::Right => imgui.set_key(2, pressed),
        Key::Up => imgui.set_key(3, pressed),
        Key::Down => imgui.set_key(4, pressed),
        Key::PageUp => imgui.set_key(5, pressed),
        Key::PageDown => imgui.set_key(6, pressed),
        Key::Home => imgui.set_key(7, pressed),
        Key::End => imgui.set_key(8, pressed),
        Key::Delete => imgui.set_key(9, pressed),
        Key::Back => imgui.set_key(10, pressed),
        Key::Return => imgui.set_key(11, pressed),
        Key::Escape => imgui.set_key(12, pressed),
        Key::A => imgui.set_key(13, pressed),
        Key::C => imgui.set_key(14, pressed),
        Key::V => imgui.set_key(15, pressed),
        Key::X => imgui.set_key(16, pressed),
        Key::Y => imgui.set_key(17, pressed),
        Key::Z => imgui.set_key(18, pressed),
        Key::LControl |
        Key::RControl => imgui.set_key_ctrl(pressed),
        Key::LShift |
        Key::RShift => imgui.set_key_shift(pressed),
        Key::LAlt | Key::RAlt => imgui.set_key_alt(pressed),
        Key::LWin | Key::RWin => imgui.set_key_super(pressed),
        _ => (),
    }
}

fn configure_keys(imgui: &mut ImGui) {
    use imgui::ImGuiKey;

    imgui.set_imgui_key(ImGuiKey::Tab, 0);
    imgui.set_imgui_key(ImGuiKey::LeftArrow, 1);
    imgui.set_imgui_key(ImGuiKey::RightArrow, 2);
    imgui.set_imgui_key(ImGuiKey::UpArrow, 3);
    imgui.set_imgui_key(ImGuiKey::DownArrow, 4);
    imgui.set_imgui_key(ImGuiKey::PageUp, 5);
    imgui.set_imgui_key(ImGuiKey::PageDown, 6);
    imgui.set_imgui_key(ImGuiKey::Home, 7);
    imgui.set_imgui_key(ImGuiKey::End, 8);
    imgui.set_imgui_key(ImGuiKey::Delete, 9);
    imgui.set_imgui_key(ImGuiKey::Backspace, 10);
    imgui.set_imgui_key(ImGuiKey::Enter, 11);
    imgui.set_imgui_key(ImGuiKey::Escape, 12);
    imgui.set_imgui_key(ImGuiKey::A, 13);
    imgui.set_imgui_key(ImGuiKey::C, 14);
    imgui.set_imgui_key(ImGuiKey::V, 15);
    imgui.set_imgui_key(ImGuiKey::X, 16);
    imgui.set_imgui_key(ImGuiKey::Y, 17);
    imgui.set_imgui_key(ImGuiKey::Z, 18);
}

fn update_mouse(imgui: &mut imgui::ImGui, mouse_state: &mut MouseState) {
    let scale = imgui.display_framebuffer_scale();
    imgui.set_mouse_pos(
        mouse_state.pos.0 as f32 / scale.0,
        mouse_state.pos.1 as f32 / scale.1,
    );
    imgui.set_mouse_down(
        &[
            mouse_state.pressed.0,
            mouse_state.pressed.1,
            mouse_state.pressed.2,
            false,
            false,
        ],
    );
    imgui.set_mouse_wheel(mouse_state.wheel / scale.1);
    mouse_state.wheel = 0.0;
}
