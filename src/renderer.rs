use wgpu::Origin3d;

use crate::infrastructure::{
    element::PlacedElement, element_tree::PlacedElementTree, widget::PlacedWidget,
};

impl PlacedElementTree<wgpu::Texture> {
    pub fn render(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
    ) -> wgpu::Texture {
        let destination = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Render Destination"),
            size: wgpu::Extent3d {
                width: self.root.rect.width(),
                height: self.root.rect.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
        });

        self.root
            .children
            .iter()
            .for_each(|element| PlacedElementTree::traverse(element, encoder, &destination));
        destination
    }

    fn traverse(
        element: &PlacedElement<wgpu::Texture>,
        encoder: &mut wgpu::CommandEncoder,
        destination: &wgpu::Texture,
    ) {
        match element {
            PlacedElement::Widget(placed_widget) => {
                PlacedElementTree::copy(encoder, placed_widget, destination)
            }
            PlacedElement::Layout(placed_layout) => placed_layout
                .children
                .iter()
                .for_each(|element| PlacedElementTree::traverse(element, encoder, destination)),
        };
    }

    // Panics:
    // When the PlacedWidget's rect's size exceeds the associated texture for the source
    fn copy(
        encoder: &mut wgpu::CommandEncoder,
        source: &PlacedWidget<wgpu::Texture>,
        destination: &wgpu::Texture,
    ) {
        encoder.copy_texture_to_texture(
            wgpu::TexelCopyTextureInfoBase {
                texture: &source.widget.texture(),
                mip_level: 1,
                origin: Origin3d::default(),
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyTextureInfoBase {
                texture: destination,
                mip_level: 1,
                origin: Origin3d::default(),
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: source.rect.width(),
                height: source.rect.height(),
                depth_or_array_layers: 1,
            },
        );
    }
}
