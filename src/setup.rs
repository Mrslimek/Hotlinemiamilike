use bevy::{
    camera::{visibility::RenderLayers, RenderTarget},
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    window::{PrimaryWindow, WindowResized},
};
use bevy_ecs_ldtk::{LdtkProjectHandle, LdtkWorldBundle};

/// Your "internal / virtual" resolution.
///
/// Everything in the game world is rendered into this resolution first,
/// then scaled up to the real window by an integer factor (with letterboxing).
pub const VIRTUAL_RES_WIDTH: u32 = 320;
pub const VIRTUAL_RES_HEIGHT: u32 = 180;

const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);
const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct OuterCamera;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    let canvas_size = Extent3d {
        width: VIRTUAL_RES_WIDTH,
        height: VIRTUAL_RES_HEIGHT,
        ..default()
    };

    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    canvas.resize(canvas_size);
    let canvas_handle = images.add(canvas);

    // In-game camera: renders the world to the low-res canvas.
    commands.spawn((
        Camera2d,
        Camera {
            // Render before the "outer" camera, so the canvas texture is ready.
            order: -1,
            target: RenderTarget::Image(canvas_handle.clone().into()),
            ..default()
        },
        Msaa::Off,
        Transform::default(),
        MainCamera,
        PIXEL_PERFECT_LAYERS,
    ));

    // The canvas sprite: displays the low-res canvas in the outer world.
    commands.spawn((Sprite::from_image(canvas_handle), HIGH_RES_LAYERS));

    // Outer camera: renders the canvas to the actual window (with black bars).
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Msaa::Off,
        Transform::default(),
        OuterCamera,
        HIGH_RES_LAYERS,
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: LdtkProjectHandle {
            handle: asset_server.load("levels/HotlineMiamiLikeWorld.ldtk"),
        },
        ..Default::default()
    });
}

/// Integer-scales the outer camera so the virtual canvas stays pixel-perfect.
pub fn fit_canvas(
    mut resized: MessageReader<WindowResized>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut projection: Single<&mut Projection, With<OuterCamera>>,
    mut did_init: Local<bool>,
) {
    let mut needs_update = false;
    for _ in resized.read() {
        needs_update = true;
    }

    if !*did_init {
        needs_update = true;
        *did_init = true;
    }

    if !needs_update {
        return;
    }

    let Projection::Orthographic(projection) = &mut **projection else {
        return;
    };

    let width = window.resolution.width();
    let height = window.resolution.height();

    let scale = (width / VIRTUAL_RES_WIDTH as f32)
        .min(height / VIRTUAL_RES_HEIGHT as f32)
        .floor()
        .max(1.0);

    projection.scale = 1.0 / scale;
}
