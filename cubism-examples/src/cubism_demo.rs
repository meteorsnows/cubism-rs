extern crate cubism;
extern crate cubism_gfx_renderer;
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate nalgebra as na;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

use cubism::model::Model;
use gfx::Device;
use gfx::Factory;
use glutin::Api::OpenGl;
use glutin::{GlContext, GlRequest};
use image::GenericImage;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new().with_gl(GlRequest::Specific(OpenGl, (3, 2)));
    let (window, mut device, mut factory, rtv, _stv) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_builder, context, &events_loop);

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

    let mut renderer = cubism_gfx_renderer::Renderer::init(&mut factory, rtv.clone()).unwrap();

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let mut running = true;

    for (idx, par) in model.parameter_ids().iter().enumerate() {
        println!("{} {}", par, idx);
    }
    model.update();

    while running {
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => running = false,
                glutin::WindowEvent::Resized(w, h) => window.resize(w, h),
                _ => (),
            },
            _ => (),
        });

        for idx in 0..model.parameter_count() {
            let par = model.parameter_values()[idx];
            model.set_parameter_value(idx, par + 0.02);
        }
        model.update();

        encoder.clear(&rtv, [0.0, 0.2, 0.0, 1.0]);

        renderer.set_mvp(na::Matrix4::<f32>::identity());
        renderer
            .draw_model(&mut factory, &mut encoder, &model, (&texture, &sampler))
            .unwrap();
        renderer.set_mvp(na::Matrix4::new_scaling(2.0));

        encoder.flush(&mut device);

        window.swap_buffers().unwrap();
        device.cleanup();
        ::std::thread::sleep_ms(16);
    }
}
