use super::{
    CameraNode, PassNode, RenderGraph, SharedBuffersNode, TextureCopyNode, WindowSwapChainNode,
    WindowTextureNode,
};
use crate::{
    pass::{
        LoadOp, Operations, PassDescriptor, RenderPassDepthStencilAttachmentDescriptor,
        TextureAttachment,
    },
    texture::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsage},
    Color,
};
use bevy_ecs::world::World;
use bevy_window::WindowId;

pub use bevy_render::render_graph::base::{camera, node, BaseRenderGraphConfig, MainPass, Msaa};

/// The "base render graph" provides a core set of render graph nodes which can be used to build any
/// graph. By itself this graph doesn't do much, but it allows Render plugins to interop with each
/// other by having a common set of nodes. It can be customized using `BaseRenderGraphConfig`.
pub(crate) fn add_base_graph(config: &BaseRenderGraphConfig, world: &mut World) {
    let world = world.cell();
    let mut graph = world.get_resource_mut::<RenderGraph>().unwrap();
    let msaa = world.get_resource::<Msaa>().unwrap();

    graph.add_node(node::TEXTURE_COPY, TextureCopyNode::default());
    if config.add_3d_camera {
        graph.add_system_node(node::CAMERA_3D, CameraNode::new(camera::CAMERA_3D));
    }

    if config.add_2d_camera {
        graph.add_system_node(node::CAMERA_2D, CameraNode::new(camera::CAMERA_2D));
    }

    graph.add_node(node::SHARED_BUFFERS, SharedBuffersNode::default());
    if config.add_main_depth_texture {
        graph.add_node(
            node::MAIN_DEPTH_TEXTURE,
            WindowTextureNode::new(
                WindowId::primary(),
                TextureDescriptor {
                    size: Extent3d {
                        depth: 1,
                        width: 1,
                        height: 1,
                    },
                    mip_level_count: 1,
                    sample_count: msaa.samples,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::Depth32Float, /* PERF: vulkan docs recommend using 24
                                                          * bit depth for better performance */
                    usage: TextureUsage::OUTPUT_ATTACHMENT,
                },
            ),
        );
    }

    if config.add_main_pass {
        let mut main_pass_node = PassNode::<&MainPass>::new(PassDescriptor {
            color_attachments: vec![msaa.color_attachment_descriptor(
                TextureAttachment::Input("color_attachment".to_string()),
                TextureAttachment::Input("color_resolve_target".to_string()),
                Operations {
                    load: LoadOp::Clear(Color::rgb(0.1, 0.1, 0.1)),
                    store: true,
                },
            )],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachmentDescriptor {
                attachment: TextureAttachment::Input("depth".to_string()),
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
            sample_count: msaa.samples,
        });

        main_pass_node.use_default_clear_color(0);

        if config.add_3d_camera {
            main_pass_node.add_camera(camera::CAMERA_3D);
        }

        if config.add_2d_camera {
            main_pass_node.add_camera(camera::CAMERA_2D);
        }

        graph.add_node(node::MAIN_PASS, main_pass_node);

        graph
            .add_node_edge(node::TEXTURE_COPY, node::MAIN_PASS)
            .unwrap();
        graph
            .add_node_edge(node::SHARED_BUFFERS, node::MAIN_PASS)
            .unwrap();

        if config.add_3d_camera {
            graph
                .add_node_edge(node::CAMERA_3D, node::MAIN_PASS)
                .unwrap();
        }

        if config.add_2d_camera {
            graph
                .add_node_edge(node::CAMERA_2D, node::MAIN_PASS)
                .unwrap();
        }
    }

    graph.add_node(
        node::PRIMARY_SWAP_CHAIN,
        WindowSwapChainNode::new(WindowId::primary()),
    );

    if config.connect_main_pass_to_swapchain {
        graph
            .add_slot_edge(
                node::PRIMARY_SWAP_CHAIN,
                WindowSwapChainNode::OUT_TEXTURE,
                node::MAIN_PASS,
                if msaa.samples > 1 {
                    "color_resolve_target"
                } else {
                    "color_attachment"
                },
            )
            .unwrap();
    }

    if msaa.samples > 1 {
        graph.add_node(
            node::MAIN_SAMPLED_COLOR_ATTACHMENT,
            WindowTextureNode::new(
                WindowId::primary(),
                TextureDescriptor {
                    size: Extent3d {
                        depth: 1,
                        width: 1,
                        height: 1,
                    },
                    mip_level_count: 1,
                    sample_count: msaa.samples,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::default(),
                    usage: TextureUsage::OUTPUT_ATTACHMENT,
                },
            ),
        );

        graph
            .add_slot_edge(
                node::MAIN_SAMPLED_COLOR_ATTACHMENT,
                WindowSwapChainNode::OUT_TEXTURE,
                node::MAIN_PASS,
                "color_attachment",
            )
            .unwrap();
    }

    if config.connect_main_pass_to_main_depth_texture {
        graph
            .add_slot_edge(
                node::MAIN_DEPTH_TEXTURE,
                WindowTextureNode::OUT_TEXTURE,
                node::MAIN_PASS,
                "depth",
            )
            .unwrap();
    }
}
