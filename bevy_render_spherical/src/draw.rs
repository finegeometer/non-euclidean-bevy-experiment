use crate::{
    pipeline::{PipelineCompiler, PipelineDescriptor, PipelineLayout, PipelineSpecialization},
    renderer::{
        AssetRenderResourceBindings, BindGroup, RenderResource, RenderResourceBinding,
        RenderResourceBindings, RenderResourceContext, SharedBuffers,
    },
    shader::Shader,
};
use bevy_asset::{Asset, Assets, Handle};
use bevy_ecs::system::{Res, ResMut, SystemParam};

pub use bevy_render::draw::{
    clear_draw_system, Draw, DrawError, OutsideFrustum, RenderCommand, Visible,
};

#[derive(SystemParam)]
pub struct DrawContext<'a> {
    pub pipelines: ResMut<'a, Assets<PipelineDescriptor>>,
    pub shaders: ResMut<'a, Assets<Shader>>,
    pub asset_render_resource_bindings: ResMut<'a, AssetRenderResourceBindings>,
    pub pipeline_compiler: ResMut<'a, PipelineCompiler>,
    pub render_resource_context: Res<'a, Box<dyn RenderResourceContext>>,
    pub shared_buffers: ResMut<'a, SharedBuffers>,
    #[system_param(ignore)]
    pub current_pipeline: Option<Handle<PipelineDescriptor>>,
}

impl<'a> DrawContext<'a> {
    pub fn get_uniform_buffer<T: RenderResource>(
        &mut self,
        render_resource: &T,
    ) -> Result<RenderResourceBinding, DrawError> {
        self.shared_buffers
            .get_uniform_buffer(&**self.render_resource_context, render_resource)
            .ok_or(DrawError::BufferAllocationFailure)
    }

    pub fn set_pipeline(
        &mut self,
        draw: &mut Draw,
        pipeline_handle: &Handle<PipelineDescriptor>,
        specialization: &PipelineSpecialization,
    ) -> Result<(), DrawError> {
        let specialized_pipeline = if let Some(specialized_pipeline) = self
            .pipeline_compiler
            .get_specialized_pipeline(pipeline_handle, specialization)
        {
            specialized_pipeline
        } else {
            self.pipeline_compiler.compile_pipeline(
                &**self.render_resource_context,
                &mut self.pipelines,
                &mut self.shaders,
                pipeline_handle,
                specialization,
            )
        };

        draw.set_pipeline(&specialized_pipeline);
        self.current_pipeline = Some(specialized_pipeline.clone_weak());
        Ok(())
    }

    pub fn get_pipeline_descriptor(&self) -> Result<&PipelineDescriptor, DrawError> {
        self.current_pipeline
            .as_ref()
            .and_then(|handle| self.pipelines.get(handle))
            .ok_or(DrawError::NoPipelineSet)
    }

    pub fn get_pipeline_layout(&self) -> Result<&PipelineLayout, DrawError> {
        self.get_pipeline_descriptor().and_then(|descriptor| {
            descriptor
                .get_layout()
                .ok_or(DrawError::PipelineHasNoLayout)
        })
    }

    pub fn set_asset_bind_groups<T: Asset>(
        &mut self,
        draw: &mut Draw,
        asset_handle: &Handle<T>,
    ) -> Result<(), DrawError> {
        if let Some(asset_bindings) = self
            .asset_render_resource_bindings
            .get_mut_untyped(&asset_handle.clone_weak_untyped())
        {
            Self::set_bind_groups_from_bindings_internal(
                &self.current_pipeline,
                &self.pipelines,
                &**self.render_resource_context,
                None,
                draw,
                &mut [asset_bindings],
            )
        } else {
            Err(DrawError::MissingAssetRenderResources)
        }
    }

    pub fn set_bind_groups_from_bindings(
        &mut self,
        draw: &mut Draw,
        render_resource_bindings: &mut [&mut RenderResourceBindings],
    ) -> Result<(), DrawError> {
        Self::set_bind_groups_from_bindings_internal(
            &self.current_pipeline,
            &self.pipelines,
            &**self.render_resource_context,
            Some(&mut self.asset_render_resource_bindings),
            draw,
            render_resource_bindings,
        )
    }

    fn set_bind_groups_from_bindings_internal(
        current_pipeline: &Option<Handle<PipelineDescriptor>>,
        pipelines: &Assets<PipelineDescriptor>,
        render_resource_context: &dyn RenderResourceContext,
        mut asset_render_resource_bindings: Option<&mut AssetRenderResourceBindings>,
        draw: &mut Draw,
        render_resource_bindings: &mut [&mut RenderResourceBindings],
    ) -> Result<(), DrawError> {
        let pipeline = current_pipeline.as_ref().ok_or(DrawError::NoPipelineSet)?;
        let pipeline_descriptor = pipelines
            .get(pipeline)
            .ok_or(DrawError::NonExistentPipeline)?;
        let layout = pipeline_descriptor
            .get_layout()
            .ok_or(DrawError::PipelineHasNoLayout)?;
        'bind_group_descriptors: for bind_group_descriptor in layout.bind_groups.iter() {
            for bindings in render_resource_bindings.iter_mut() {
                if let Some(bind_group) =
                    bindings.update_bind_group(bind_group_descriptor, render_resource_context)
                {
                    draw.set_bind_group(bind_group_descriptor.index, bind_group);
                    continue 'bind_group_descriptors;
                }
            }

            // if none of the given RenderResourceBindings have the current bind group, try their
            // assets
            let asset_render_resource_bindings =
                if let Some(value) = asset_render_resource_bindings.as_mut() {
                    value
                } else {
                    continue 'bind_group_descriptors;
                };
            for bindings in render_resource_bindings.iter_mut() {
                for (asset_handle, _) in bindings.iter_assets() {
                    let asset_bindings = if let Some(asset_bindings) =
                        asset_render_resource_bindings.get_mut_untyped(asset_handle)
                    {
                        asset_bindings
                    } else {
                        continue;
                    };

                    if let Some(bind_group) = asset_bindings
                        .update_bind_group(bind_group_descriptor, render_resource_context)
                    {
                        draw.set_bind_group(bind_group_descriptor.index, bind_group);
                        continue 'bind_group_descriptors;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn create_bind_group_resource(
        &self,
        index: u32,
        bind_group: &BindGroup,
    ) -> Result<(), DrawError> {
        let pipeline = self
            .current_pipeline
            .as_ref()
            .ok_or(DrawError::NoPipelineSet)?;
        let pipeline_descriptor = self
            .pipelines
            .get(pipeline)
            .ok_or(DrawError::NonExistentPipeline)?;
        let layout = pipeline_descriptor
            .get_layout()
            .ok_or(DrawError::PipelineHasNoLayout)?;
        let bind_group_descriptor = &layout.bind_groups[index as usize];
        self.render_resource_context
            .create_bind_group(bind_group_descriptor.id, bind_group);
        Ok(())
    }

    pub fn set_vertex_buffers_from_bindings(
        &self,
        draw: &mut Draw,
        render_resource_bindings: &[&RenderResourceBindings],
    ) -> Result<(), DrawError> {
        for bindings in render_resource_bindings.iter() {
            if let Some((index_buffer, index_format)) = bindings.index_buffer {
                draw.set_index_buffer(index_buffer, 0, index_format);
            }
            if let Some(main_vertex_buffer) = bindings.vertex_attribute_buffer {
                draw.set_vertex_buffer(0, main_vertex_buffer, 0);
            }
        }
        Ok(())
    }
}

pub trait Drawable {
    fn draw(&mut self, draw: &mut Draw, context: &mut DrawContext) -> Result<(), DrawError>;
}
