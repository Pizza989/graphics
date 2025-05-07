use std::{path::Path, rc::Rc};

use graphics::{
    geometry::Size,
    infrastructure::{
        element::{ComposedElement, Element},
        element_tree::ElementTree,
        layout::Layout,
    },
};

use wgpu::*;

pub async fn save_texture_to_png(
    device: &Device,
    queue: &Queue,
    mut encoder: CommandEncoder,
    texture: &Texture,
    texture_size: Extent3d,
    path: &std::path::Path,
) {
    // we need to store this for later
    let padded_bytes_per_row = (((texture_size.width * 4) + 255) / 256) * 256;
    let bytes_per_image = padded_bytes_per_row * texture_size.height;
    let output_buffer_size = (bytes_per_image * texture_size.depth_or_array_layers) as u64;

    let output_buffer_desc = wgpu::BufferDescriptor {
        size: output_buffer_size,
        usage: wgpu::BufferUsages::COPY_DST
        // this tells wpgu that we want to read this buffer from the cpu
        | wgpu::BufferUsages::MAP_READ,
        label: None,
        mapped_at_creation: false,
    };
    let output_buffer = device.create_buffer(&output_buffer_desc);

    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            aspect: wgpu::TextureAspect::All,
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &output_buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded_bytes_per_row),
                rows_per_image: None,
            },
        },
        texture_size,
    );

    queue.submit(Some(encoder.finish()));

    // We need to scope the mapping variables so that we can
    // unmap the buffer
    {
        let buffer_slice = output_buffer.slice(..);

        // NOTE: We have to create the mapping THEN device.poll() before await
        // the future. Otherwise the application will freeze.
        let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::PollType::Wait).unwrap();
        rx.receive().await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();

        use image::{ImageBuffer, Rgba};
        let buffer =
            ImageBuffer::<Rgba<u8>, _>::from_raw(texture_size.width, texture_size.height, data)
                .unwrap();
        buffer.save(path).unwrap();
    }
    output_buffer.unmap();
}

struct BasicLayout {
    children: Vec<Element<wgpu::Texture>>,
}

impl Layout for BasicLayout {
    type Texture = wgpu::Texture;

    fn children(&self) -> &Vec<Element<Self::Texture>> {
        &self.children
    }

    fn composite(&self, size: Size) -> Vec<ComposedElement<Self::Texture>> {
        vec![]
    }
}

async fn init_wgpu() -> (wgpu::Device, wgpu::Queue) {
    let instance_desc = InstanceDescriptor::default();
    let instance = Instance::new(&instance_desc);
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();
    adapter.request_device(&Default::default()).await.unwrap()
}

fn main() {
    let (device, queue) = pollster::block_on(init_wgpu());
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    let mut etree = ElementTree::new(Rc::new(BasicLayout { children: vec![] }));
    etree.composite((720, 480).into());

    if let Some(placed_tree) = etree.placed_tree {
        let gui_texture = placed_tree.render(&device, &mut encoder);

        pollster::block_on(save_texture_to_png(
            &device,
            &queue,
            encoder,
            &gui_texture,
            Extent3d {
                width: 720,
                height: 480,
                depth_or_array_layers: 1,
            },
            Path::new("image.png"),
        ));
    }
}
