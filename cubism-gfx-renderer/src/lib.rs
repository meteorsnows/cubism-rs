extern crate cubism;
#[macro_use]
extern crate gfx;
extern crate nalgebra as na;

use cubism::model::Model;
use na::Matrix4;

use gfx::handle::{Buffer, RenderTargetView};
use gfx::memory::Bind;
use gfx::texture::{FilterMethod, SamplerInfo, WrapMode};
use gfx::traits::FactoryExt;
use gfx::TextureSampler;
use gfx::{Bundle, CommandBuffer, Encoder, Factory, IntoIndexBuffer, Resources, Slice};

#[derive(Copy, Clone)]
pub struct Color(f32, f32, f32, f32);

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color(r, g, b, a)
    }

    pub fn r(&self) -> f32 {
        self.0
    }
    pub fn g(&self) -> f32 {
        self.1
    }
    pub fn b(&self) -> f32 {
        self.2
    }
    pub fn a(&self) -> f32 {
        self.3
    }

    pub fn set_r(&mut self, val: f32) {
        self.0 = val;
    }
    pub fn set_g(&mut self, val: f32) {
        self.1 = val;
    }
    pub fn set_b(&mut self, val: f32) {
        self.2 = val;
    }
    pub fn set_a(&mut self, val: f32) {
        self.3 = val;
    }
}

type ColorFormat = gfx::format::Rgba8;
pub type RendererResult<T> = Result<T, RendererError>;

#[derive(Clone, Debug)]
pub enum RendererError {
    Update(gfx::UpdateError<usize>),
    Buffer(gfx::buffer::CreationError),
    Pipeline(gfx::PipelineStateError<String>),
    Combined(gfx::CombinedError),
}

impl From<gfx::UpdateError<usize>> for RendererError {
    fn from(e: gfx::UpdateError<usize>) -> RendererError {
        RendererError::Update(e)
    }
}

impl From<gfx::buffer::CreationError> for RendererError {
    fn from(e: gfx::buffer::CreationError) -> RendererError {
        RendererError::Buffer(e)
    }
}

impl From<gfx::PipelineStateError<String>> for RendererError {
    fn from(e: gfx::PipelineStateError<String>) -> RendererError {
        RendererError::Pipeline(e)
    }
}

impl From<gfx::CombinedError> for RendererError {
    fn from(e: gfx::CombinedError) -> RendererError {
        RendererError::Combined(e)
    }
}

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_pos",
        tex_coord: [f32; 2] = "a_tex_coord",
        color: [f32; 3] = "a_color",
    }

    pipeline pipe {
        vertex_buffer: gfx::VertexBuffer<Vertex> = (),
        tex: TextureSampler<[f32; 4]> = "tex",
        mvp: gfx::Global<[[f32; 4]; 4]> = "u_mvp",
        out: gfx::BlendTarget<ColorFormat> = (
            "Target0",
            gfx::state::ColorMask::all(),
            gfx::preset::blend::ALPHA,
        ),
    }
}

pub enum BlendMode {
    Normal,
    Additive,
    Multiplicative,
}

fn copy_unsized_to_fixedsize(mat: &[f32], slice: &mut [[f32; 4]; 4]) {
    slice[0].copy_from_slice(&mat[0..4]);
    slice[1].copy_from_slice(&mat[4..8]);
    slice[2].copy_from_slice(&mat[8..12]);
    slice[3].copy_from_slice(&mat[12..16]);
}

pub struct Renderer<R: Resources> {
    bundle: Bundle<R, pipe::Data<R>>,
    index_buffer: Buffer<R, u16>,
    color: Color,
    mvp: Matrix4<f32>,
}

impl<R: Resources> Renderer<R> {
    pub fn init<F: Factory<R>>(
        factory: &mut F,
        target: RenderTargetView<R, ColorFormat>,
    ) -> RendererResult<Self> {
        let pso = factory.create_pipeline_simple(
            include_bytes!("../shader/gl/330.vert"),
            include_bytes!("../shader/gl/330.frag"),
            pipe::new(),
        )?;
        let vertex_buffer = factory.create_buffer::<Vertex>(
            256,
            gfx::buffer::Role::Vertex,
            gfx::memory::Usage::Dynamic,
            Bind::empty(),
        )?;
        let index_buffer = factory.create_buffer::<u16>(
            256,
            gfx::buffer::Role::Index,
            gfx::memory::Usage::Dynamic,
            Bind::empty(),
        )?;
        let (_, texture) = factory
            .create_texture_immutable_u8::<gfx::format::Rgba8>(
                gfx::texture::Kind::D2(2, 2, gfx::texture::AaMode::Single),
                gfx::texture::Mipmap::Provided,
                &[&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]],
            )
            .unwrap();
        let sampler =
            factory.create_sampler(SamplerInfo::new(FilterMethod::Scale, WrapMode::Clamp));
        let mut data = pipe::Data {
            vertex_buffer,
            tex: (texture, sampler),
            mvp: [[0.0; 4]; 4],
            out: target,
        };
        copy_unsized_to_fixedsize(Matrix4::<f32>::identity().as_slice(), &mut data.mvp);
        let slice = Slice {
            start: 0,
            end: 0,
            base_vertex: 0,
            instances: None,
            buffer: index_buffer.clone().into_index_buffer(factory),
        };
        Ok(Renderer {
            bundle: Bundle::new(slice, pso, data),
            index_buffer,
            color: Color(1.0, 1.0, 1.0, 1.0),
            mvp: Matrix4::identity(),
        })
    }

    pub fn draw_model<F: Factory<R>, C: CommandBuffer<R>>(
        &mut self,
        factory: &mut F,
        encoder: &mut Encoder<R, C>,
        model: &Model,
        (texture, sampler): (
            &gfx::handle::ShaderResourceView<R, [f32; 4]>,
            &gfx::handle::Sampler<R>,
        ),
    ) -> RendererResult<()> {
        self.bundle.data.tex = (texture.clone(), sampler.clone());

        let mut sorted_draw_indices = vec![0; model.drawable_count()];
        for (idx, order) in model.drawable_render_orders().iter().enumerate() {
            sorted_draw_indices[*order as usize] = idx;
        }

        copy_unsized_to_fixedsize(self.mvp.as_slice(), &mut self.bundle.data.mvp);

        for draw_idx in sorted_draw_indices {
            //set clipping mask

            self.draw_mesh(factory, encoder, model, draw_idx).unwrap();
        }
        Ok(())
    }

    fn draw_mesh<F: Factory<R>, C: CommandBuffer<R>>(
        &mut self,
        factory: &mut F,
        encoder: &mut Encoder<R, C>,
        model: &Model,
        index: usize,
    ) -> RendererResult<()> {
        let opacity = model.drawable_opacities()[index];
        if opacity <= 0.0 {
            return Ok(());
        }
        //let vtx_index_count = model.drawable_index_counts()[index];
        let vtx_pos = model.drawable_vertex_positions(index);
        let vtx_uv = model.drawable_vertex_uvs(index);
        let mut vtx_buffer = Vec::with_capacity(vtx_pos.len());
        for i in 0..vtx_pos.len() {
            let vtx_pos = vtx_pos[i];
            let vtx_uv = vtx_uv[i];
            vtx_buffer.push(Vertex {
                pos: [vtx_pos.0, vtx_pos.1],
                tex_coord: [vtx_uv.0, vtx_uv.1],
                color: [1.0, 1.0, 1.0],
            });
        }
        let idx_buffer = Vec::from(model.drawable_indices(index));
        self.upload_vertex_buffer(factory, encoder, &vtx_buffer)?;
        self.upload_index_buffer(factory, encoder, &idx_buffer)?;

        self.bundle.slice.end = idx_buffer.len() as u32;
        self.bundle.encode(encoder);

        Ok(())
    }

    fn upload_vertex_buffer<F: Factory<R>, C: CommandBuffer<R>>(
        &mut self,
        factory: &mut F,
        encoder: &mut Encoder<R, C>,
        vtx_buffer: &[Vertex],
    ) -> RendererResult<()> {
        if self.bundle.data.vertex_buffer.len() < vtx_buffer.len() {
            self.bundle.data.vertex_buffer = factory.create_buffer::<Vertex>(
                vtx_buffer.len(),
                gfx::buffer::Role::Vertex,
                gfx::memory::Usage::Dynamic,
                Bind::empty(),
            )?;
        }
        encoder.update_buffer(&self.bundle.data.vertex_buffer, vtx_buffer, 0)?;
        Ok(())
    }

    fn upload_index_buffer<F: Factory<R>, C: CommandBuffer<R>>(
        &mut self,
        factory: &mut F,
        encoder: &mut Encoder<R, C>,
        idx_buffer: &[u16],
    ) -> RendererResult<()> {
        if self.index_buffer.len() < idx_buffer.len() {
            self.index_buffer = factory.create_buffer::<u16>(
                idx_buffer.len(),
                gfx::buffer::Role::Index,
                gfx::memory::Usage::Dynamic,
                Bind::empty(),
            )?;
            self.bundle.slice.buffer = self.index_buffer.clone().into_index_buffer(factory);
        }
        encoder.update_buffer(&self.index_buffer, idx_buffer, 0)?;
        Ok(())
    }

    pub fn mvp(&self) -> Matrix4<f32> {
        self.mvp
    }

    pub fn mvp_mut(&mut self) -> &mut Matrix4<f32> {
        &mut self.mvp
    }

    pub fn set_mvp(&mut self, mat: Matrix4<f32>) {
        self.mvp = mat;
    }

    pub fn model_color(&self) -> Color {
        self.color
    }

    pub fn set_model_color(&mut self, c: Color) {
        self.color = c;
    }
}
