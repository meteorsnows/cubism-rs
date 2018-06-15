extern crate cubism;
extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;
extern crate nalgebra as na;
extern crate image;
extern crate cubism_gfx_renderer;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

use gfx::traits::FactoryExt;
use gfx::Factory;
use gfx::Device;
use glutin::{GlContext, GlRequest};
use glutin::Api::OpenGl;
use image::GenericImage;
use cubism::moc::Moc;
use cubism::model::Model;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(OpenGl, (3, 2)));
    let (window, mut device, mut factory, rtv, stv) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_builder, context, &events_loop);

    let mut model = Model::from_bytes(include_bytes!("../res/Mark.moc3")).unwrap();
    let mut model2 = Model::from_bytes(include_bytes!("../res/Koharu.moc3")).unwrap();
    let mut im = image::open("cubism-examples/res/Koharu.png").unwrap().flipv();
    let (_, texture) = factory.create_texture_immutable_u8::<gfx::format::Rgba8>(
        gfx::texture::Kind::D2(
            im.width() as u16,
            im.height() as u16,
            gfx::texture::AaMode::Single,
        ),
        gfx::texture::Mipmap::Provided,
        &[&im.raw_pixels()]).unwrap();
    let sampler =
        factory.create_sampler(gfx::texture::SamplerInfo::new(gfx::texture::FilterMethod::Scale, gfx::texture::WrapMode::Clamp));

    let mut im = image::open("cubism-examples/res/Mark.png").unwrap().flipv();
    let (_, texture2) = factory.create_texture_immutable_u8::<gfx::format::Rgba8>(
        gfx::texture::Kind::D2(
            im.width() as u16,
            im.height() as u16,
            gfx::texture::AaMode::Single,
        ),
        gfx::texture::Mipmap::Provided,
        &[&im.raw_pixels()]).unwrap();
    let sampler2 =
        factory.create_sampler(gfx::texture::SamplerInfo::new(gfx::texture::FilterMethod::Scale, gfx::texture::WrapMode::Clamp));

    let mut renderer = cubism_gfx_renderer::Renderer::init(&mut factory, rtv.clone()).unwrap();

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let mut running = true;

    for (idx, par) in model2.parameter_ids().iter().enumerate() {
        println!("{} {}", par, idx);

    }
    model2.update();

    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(w, h) => window.resize(w, h),
                    _ => ()
                },
                _ => ()
            }
        });

        for idx in 0..model2.parameter_count() {
            let par = model2.parameter_values()[idx];
            let max = model2.parameter_max()[idx];
            let min = model2.parameter_min()[idx];
            model2.set_parameter_value(idx, par + 0.02);//change value space to be bewtten max min
        }
        model2.update();

        //renderer.draw_model(&model);
        encoder.clear(&rtv, [0.0, 0.2, 0.0, 1.0]); //clear the framebuffer with a color(color needs to be an array of 4 f32s, RGBa)

        renderer.set_mvp(na::Matrix4::<f32>::identity());
        renderer.draw_model(&mut factory, &mut encoder, &model2, (&texture, &sampler)).unwrap();
        //renderer.draw_model(&mut factory, &mut encoder, &model, (&texture2, &sampler2)).unwrap();
        renderer.set_mvp(na::Matrix4::new_scaling(2.0));
        //renderer.draw_model(&mut factory, &mut encoder, &model).unwrap();

        encoder.flush(&mut device); // execute draw commands


        window.swap_buffers().unwrap();
        device.cleanup();
        ::std::thread::sleep_ms(16);
    }
}
