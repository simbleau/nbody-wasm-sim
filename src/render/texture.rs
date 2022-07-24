use image::{GenericImageView, RgbaImage};
use wgpu::{
    BindGroup, BindGroupLayout, Device, Extent3d, Sampler, TextureView,
};

pub struct Texture {
    name: &'static str,
    pub rgba: RgbaImage,
    pub dimensions: (u32, u32),
    pub size: Extent3d,
}

impl Texture {
    pub fn new(name: &'static str, bytes: &'static [u8]) -> Self {
        let image = image::load_from_memory(bytes)
            .expect("Could not load image memory");
        let rgba = image.to_rgba8();
        let dimensions = image.dimensions();
        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        Texture {
            name,
            rgba,
            dimensions,
            size: texture_size,
        }
    }

    pub fn bind(
        &self,
        device: &Device,
    ) -> (wgpu::Texture, BindGroup, BindGroupLayout) {
        let (texture, texture_view) = self.create_texture(device);
        let layout = self.create_bind_group_layout(device);
        let sampler = self.create_sampler(device);
        let bind_group =
            self.create_bind_group(device, &layout, &texture_view, &sampler);

        (texture, bind_group, layout)
    }

    fn create_texture(&self, device: &Device) -> (wgpu::Texture, TextureView) {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: self.size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST,
            label: Some(self.name),
        });

        let texture_view =
            texture.create_view(&wgpu::TextureViewDescriptor::default());

        (texture, texture_view)
    }

    fn create_sampler(&self, device: &Device) -> Sampler {
        device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        })
    }

    fn create_bind_group(
        &self,
        device: &Device,
        layout: &BindGroupLayout,
        texture_view: &TextureView,
        sampler: &Sampler,
    ) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
            label: Some(self.name),
        })
    }

    fn create_bind_group_layout(&self, device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float {
                            filterable: true,
                        },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(
                        wgpu::SamplerBindingType::Filtering,
                    ),
                    count: None,
                },
            ],
            label: Some(self.name),
        })
    }
}
