use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    math::{
        bounding::{Aabb2d, BoundingVolume},
        vec2, Rect,
    },
    prelude::*,
    render::camera::CameraProjection,
    window::PrimaryWindow,
};
use std::ops::RangeInclusive;
use bevy::input::touch;

/// This is the tag that the plugin will scan for and update its `Camera` component.
/// You can either attach it manually to your camera, or the plugin will try to attach it
/// to the default camera in the `PostStartup` schedule
#[derive(Component)]
pub struct TouchCameraTag;

/// Plugin that adds the necessary systems for `PanCam` components to work
#[derive(Default)]
pub struct PanCamPlugin;

/// System set to allow ordering of `PanCamPlugin`
#[derive(Debug, Clone, Copy, SystemSet, PartialEq, Eq, Hash)]
pub struct PanCamSystemSet;

/// Which keys move the camera in particular directions for keyboard movement
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct DirectionKeys {
    ///  The keys that move the camera up
    pub up: Vec<KeyCode>,
    ///  The keys that move the camera down
    pub down: Vec<KeyCode>,
    ///  The keys that move the camera left
    pub left: Vec<KeyCode>,
    ///  The keys that move the camera right
    pub right: Vec<KeyCode>,
}

impl DirectionKeys {
    /// No keys move the camera
    pub const NONE: Self = Self {
        up: vec![],
        down: vec![],
        left: vec![],
        right: vec![],
    };

    /// The camera is moved by the arrow keys
    pub fn arrows() -> Self {
        Self {
            up: vec![KeyCode::ArrowUp],
            down: vec![KeyCode::ArrowDown],
            left: vec![KeyCode::ArrowLeft],
            right: vec![KeyCode::ArrowRight],
        }
    }

    /// The camera is moved by the WASD keys
    pub fn wasd() -> Self {
        Self {
            up: vec![KeyCode::KeyW],
            down: vec![KeyCode::KeyS],
            left: vec![KeyCode::KeyA],
            right: vec![KeyCode::KeyD],
        }
    }

    /// The camera is moved by the arrow and WASD keys
    pub fn arrows_and_wasd() -> Self {
        Self {
            up: vec![KeyCode::ArrowUp, KeyCode::KeyW],
            down: vec![KeyCode::ArrowDown, KeyCode::KeyS],
            left: vec![KeyCode::ArrowLeft, KeyCode::KeyA],
            right: vec![KeyCode::ArrowRight, KeyCode::KeyD],
        }
    }

    fn direction(&self, keyboard_buttons: &Res<ButtonInput<KeyCode>>) -> Vec2 {
        let mut direction = Vec2::ZERO;

        if self.left.iter().any(|key| keyboard_buttons.pressed(*key)) {
            direction.x -= 1.;
        }

        if self.right.iter().any(|key| keyboard_buttons.pressed(*key)) {
            direction.x += 1.;
        }

        if self.up.iter().any(|key| keyboard_buttons.pressed(*key)) {
            direction.y += 1.;
        }

        if self.down.iter().any(|key| keyboard_buttons.pressed(*key)) {
            direction.y -= 1.;
        }

        direction
    }
}

#[derive(PartialEq, Default, Reflect)]
enum GestureType {
    #[default]
    None,
    Pan,
    Pinch,
    PinchCancelled,
}

#[derive(Resource, Default)]
struct TouchTracker {
    pub camera_start_pos: Vec3,
    pub time_start_touch: f32,
    pub gesture_type: GestureType,

    // Keeps track of position on last frame.
    // This is different from Touch.last_position as that only updates when there has been a movement
    pub last_touch_a: Option<Vec2>,
    pub last_touch_b: Option<Vec2>,
}

impl Plugin for PanCamPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(TouchTracker::default())
            .add_systems(
                Update,
                (do_camera_movement, on_mouse_wheel, touch_pan_zoom).in_set(PanCamSystemSet),
            )
            .register_type::<PanCam>()
            .register_type::<DirectionKeys>();
    }
}

/// 鼠标滚轮事件, 触发为平移
/// 按住 shift 键，则为 x 轴平移
/// 按住 ctrl 键，则为缩放
/// 系统触摸板分发的鼠标滚轮事件，默认均为缩放比的倍数 作为平移事件处理
/// 双指放大缩小则x=0,y为其他小数, 作为缩放事件处理
///
fn on_mouse_wheel(
    query: Query<(
        &PanCam,
        &Camera,
        &mut OrthographicProjection,
        &mut Transform,
    )>,
    scroll_events: EventReader<MouseWheel>,
    keys: Res<ButtonInput<KeyCode>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    let mut scroll_offset = scroll_offset_from_events(scroll_events);
    if scroll_offset.x == 0.0 && scroll_offset.y == 0.0 {
        return;
    }
    let Ok(window) = primary_window.get_single() else {
        return;
    };
    let scale_factor = window.scale_factor();
    let mut is_zoom = false;
    let is_pan = true;
    let is_pressed_ctrl = keys.any_pressed(vec![KeyCode::ControlLeft, KeyCode::ControlRight]);
    let is_zoom_by_touch_pad = scroll_offset.y % scale_factor != 0.0;
    if is_pressed_ctrl {
        // 按下 ctrl 键，则缩放
        is_zoom = true;
    } else if is_zoom_by_touch_pad {
        is_zoom = true;
    }
    if is_zoom {
        do_camera_zoom(query, primary_window, scroll_offset.y, is_zoom_by_touch_pad)
    } else if is_pan {
        let swap = keys.any_pressed(vec![KeyCode::ShiftLeft, KeyCode::ShiftRight]) && scroll_offset.x == 0.;
        if swap {
            scroll_offset.x = scroll_offset.y;
            scroll_offset.y = 0.;
        }
        do_camera_pan_mouse_wheel(query, scroll_offset)
    }
    debug!("is_zoom:{}, is_pan:{}, scroll_offset:{}", is_zoom, is_pan, scroll_offset)
}

fn do_camera_zoom(
    mut query: Query<(
        &PanCam,
        &Camera,
        &mut OrthographicProjection,
        &mut Transform,
    )>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    scroll_offset: f32,
    is_zoom_by_touch_pad: bool,
) {
    const ZOOM_SENSITIVITY: f32 = 0.001;

    let Ok(window) = primary_window.get_single() else {
        return;
    };

    for (pan_cam, camera, mut proj, mut transform) in &mut query {
        if !pan_cam.enabled {
            continue;
        }
        let view_size = camera.logical_viewport_size().unwrap_or(window.size());

        let old_scale = proj.scale;
        let mut offset = scroll_offset;
        if is_zoom_by_touch_pad {
            offset *= pan_cam.touch_pad_zoom_scale;
        }
        proj.scale *= 1. - offset * ZOOM_SENSITIVITY;

        constrain_proj_scale(
            &mut proj,
            pan_cam.rect().size(),
            &pan_cam.scale_range(),
            view_size,
        );

        let cursor_normalized_viewport_pos = window
            .cursor_position()
            .map(|cursor_pos| {
                let view_pos = camera
                    .logical_viewport_rect()
                    .map(|v| v.min)
                    .unwrap_or(Vec2::ZERO);

                ((cursor_pos - view_pos) / view_size) * 2. - Vec2::ONE
            })
            .map(|p| vec2(p.x, -p.y));

        // Move the camera position to normalize the projection window
        let (Some(cursor_normalized_view_pos), true) =
            (cursor_normalized_viewport_pos, pan_cam.zoom_to_cursor)
        else {
            continue;
        };

        let proj_size = proj.area.max / old_scale;

        let cursor_world_pos =
            transform.translation.truncate() + cursor_normalized_view_pos * proj_size * old_scale;

        let proposed_cam_pos =
            cursor_world_pos - cursor_normalized_view_pos * proj_size * proj.scale;

        // As we zoom out, we don't want the viewport to move beyond the provided
        // boundary. If the most recent change to the camera zoom would move cause
        // parts of the window beyond the boundary to be shown, we need to change the
        // camera position to keep the viewport within bounds.
        transform.translation =
            clamp_to_safe_zone(proposed_cam_pos, pan_cam.aabb(), proj.area.size())
                .extend(transform.translation.z);
    }
}

fn do_camera_pan_mouse_wheel(
    mut query: Query<(
        &PanCam,
        &Camera,
        &mut OrthographicProjection,
        &mut Transform,
    )>,
    mut offset: Vec2,
) {
    for (pan_cam, _, proj, mut transform) in &mut query {
        if !pan_cam.enabled {
            continue;
        }
        if offset.x.abs() >= 100.0 {
            // 鼠标事件, * 缩放值
            offset.x *= pan_cam.mouse_wheel_pan_scale;
        }
        if offset.y.abs() >= 100.0 {
            // 鼠标事件, * 缩放值
            offset.y *= pan_cam.mouse_wheel_pan_scale;
        }
        transform.translation.x -= offset.x * proj.scale;
        transform.translation.y += offset.y * proj.scale;
    }
}

/// Consumes `MouseWheel` event reader and calculates a single scalar,
/// representing positive or negative scroll offset.
fn scroll_offset_from_events(mut scroll_events: EventReader<MouseWheel>) -> Vec2 {
    let pixels_per_line = 100.; // Maybe make configurable?
    let mut vec2 = Vec2::new(0.0, 0.0);
    for event in scroll_events.read() {
        match event.unit {
            MouseScrollUnit::Pixel => {
                vec2.x += event.x;
                vec2.y += event.y;
            }
            MouseScrollUnit::Line => {
                vec2.x += event.x * pixels_per_line;
                vec2.y += event.y * pixels_per_line;
            }
        }
    }
    vec2
}

/// `max_scale_within_bounds` is used to find the maximum safe zoom out/projection
/// scale when we have been provided with minimum and maximum x boundaries for
/// the camera.
fn max_scale_within_bounds(
    bounded_area_size: Vec2,
    proj: &OrthographicProjection,
    window_size: Vec2, //viewport?
) -> Vec2 {
    let mut proj = proj.clone();
    proj.scale = 1.;
    proj.update(window_size.x, window_size.y);
    let base_world_size = proj.area.size();
    bounded_area_size / base_world_size
}

/// Makes sure that the camera projection scale stays in the provided bounds
/// and range.
fn constrain_proj_scale(
    proj: &mut OrthographicProjection,
    bounded_area_size: Vec2,
    scale_range: &RangeInclusive<f32>,
    window_size: Vec2,
) {
    proj.scale = proj.scale.clamp(*scale_range.start(), *scale_range.end());

    // If there is both a min and max boundary, that limits how far we can zoom.
    // Make sure we don't exceed that
    if bounded_area_size.x.is_finite() || bounded_area_size.y.is_finite() {
        let max_safe_scale = max_scale_within_bounds(bounded_area_size, proj, window_size);
        proj.scale = proj.scale.min(max_safe_scale.x).min(max_safe_scale.y);
    }
}

/// Clamps a camera position to a safe zone. "Safe" means that each screen
/// corner is constrained to the corresponding bound corner.
///
/// Since bevy doesn't provide a `shrink` method on a `Rect` yet, we have to
/// operate on `Aabb2d` type.
fn clamp_to_safe_zone(pos: Vec2, aabb: Aabb2d, bounded_area_size: Vec2) -> Vec2 {
    let aabb = aabb.shrink(bounded_area_size / 2.);
    pos.clamp(aabb.min, aabb.max)
}

fn do_camera_movement(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard_buttons: Res<ButtonInput<KeyCode>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&PanCam, &Camera, &mut Transform, &OrthographicProjection)>,
    mut last_pos: Local<Option<Vec2>>,
    time: Res<Time>,
) {
    let Ok(window) = primary_window.get_single() else {
        return;
    };
    if !keys.any_pressed(vec![KeyCode::Space]) {
        // TODO 鼠标样式
        // 按住空格键 平移
        return;
    }
    let window_size = window.size();

    // Use position instead of MouseMotion, otherwise we don't get acceleration
    // movement
    let current_pos = match window.cursor_position() {
        Some(c) => vec2(c.x, -c.y),
        None => return,
    };
    let delta_device_pixels = current_pos - last_pos.unwrap_or(current_pos);

    for (pan_cam, camera, mut transform, projection) in &mut query {
        if !pan_cam.enabled {
            continue;
        }

        let proj_area_size = projection.area.size();

        let mouse_delta = if !pan_cam
            .grab_buttons
            .iter()
            .any(|btn| mouse_buttons.pressed(*btn) && !mouse_buttons.just_pressed(*btn))
        {
            Vec2::ZERO
        } else {
            let viewport_size = camera.logical_viewport_size().unwrap_or(window_size);
            delta_device_pixels * proj_area_size / viewport_size
        };

        let direction = pan_cam.move_keys.direction(&keyboard_buttons);

        let keyboard_delta =
            time.delta_seconds() * direction.normalize_or_zero() * pan_cam.speed * projection.scale;
        let delta = mouse_delta - keyboard_delta;

        if delta == Vec2::ZERO {
            continue;
        }

        // The proposed new camera position
        let proposed_cam_pos = transform.translation.truncate() - delta;

        transform.translation =
            clamp_to_safe_zone(proposed_cam_pos, pan_cam.aabb(), proj_area_size)
                .extend(transform.translation.z);
    }
    *last_pos = Some(current_pos);
}

fn touch_pan_zoom(
    mut query: Query<(&PanCam, &Camera, &mut Transform, &mut OrthographicProjection)>,
    touches_res: Res<Touches>,
    mut tracker: ResMut<TouchTracker>,
    time: Res<Time>,
) {
    for (pan_cam, _, mut transform, mut projection) in &mut query {
        if !pan_cam.enabled {
            continue;
        }
        let touches: Vec<&touch::Touch> = touches_res.iter().collect();

        if touches.is_empty() {
            tracker.gesture_type = GestureType::None;
            tracker.last_touch_a = None;
            tracker.last_touch_b = None;
            return;
        }

        if touches_res.any_just_released() {
            tracker.gesture_type = GestureType::PinchCancelled;
            tracker.last_touch_a = None;
            tracker.last_touch_b = None;
        }

        if touches.len() == 2 {
            // TODO 以双指中心区域放大 同时处理双指选中的平移
            tracker.gesture_type = GestureType::Pinch;
            // complicated way to reset previous position to prevent some bugs. Should simplify
            let last_a = if tracker.last_touch_b.is_none() {
                touches[0].position()
            } else {
                tracker.last_touch_a.unwrap_or(touches[0].position())
            };
            let last_b = if tracker.last_touch_b.is_none() {
                touches[1].position()
            } else {
                tracker.last_touch_b.unwrap_or(touches[1].position())
            };

            let delta_a = touches[0].position() - last_a;
            let delta_b = touches[1].position() - last_b;
            let delta_total = (delta_a + delta_b).length();
            let dot_delta = delta_a.dot(delta_b);
            if dot_delta > pan_cam.touch_config.opposites_tolerance {
                return;
            }

            let distance_current = touches[0].position() - touches[1].position();
            let distance_prev = touches[0].previous_position() - touches[1].previous_position();
            let pinch_direction = distance_prev.length() - distance_current.length();
            let scale = pinch_direction.signum() * delta_total * pan_cam.touch_config.zoom_sensitivity * projection.scale;
            projection.scale += scale;
            if projection.scale > pan_cam.max_scale {
                projection.scale = pan_cam.max_scale;
            }
            if projection.scale < pan_cam.min_scale {
                projection.scale = pan_cam.min_scale;
            }
            tracker.last_touch_a = Some(touches[0].position());
            tracker.last_touch_b = Some(touches[1].position());
        } else if touches.len() == 1
            && matches!(tracker.gesture_type, GestureType::None | GestureType::Pan)
        {
            // TODO 拖拽与选择事件通过开关控制
            if tracker.gesture_type == GestureType::None {
                tracker.camera_start_pos = transform.translation;
                tracker.time_start_touch = time.elapsed_seconds();
            }
            tracker.gesture_type = GestureType::Pan;
            let time_since_start = time.elapsed_seconds() - tracker.time_start_touch;
            if time_since_start < pan_cam.touch_config.touch_time_min {
                return;
            }
            let distance = Vec3::new(touches[0].distance().x, -touches[0].distance().y, 0.);
            transform.translation =
                tracker.camera_start_pos - pan_cam.touch_config.drag_sensitivity * distance * projection.scale;
            tracker.last_touch_a = Some(touches[0].position());
            tracker.last_touch_b = None;
        }
    }
}

/// A component that adds panning camera controls to an orthographic camera
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PanCam {
    /// The mouse buttons that will be used to drag and pan the camera
    pub grab_buttons: Vec<MouseButton>,
    /// The keyboard keys that will be used to move the camera
    pub move_keys: DirectionKeys,
    /// Speed for keyboard movement
    ///
    /// This is multiplied with the projection scale of the camera so the
    /// speed stays proportional to the current "zoom" level
    pub speed: f32,
    /// Whether camera currently responds to user input
    pub enabled: bool,
    /// When true, zooming the camera will center on the mouse cursor
    ///
    /// When false, the camera will stay in place, zooming towards the
    /// middle of the screen
    pub zoom_to_cursor: bool,
    /// The minimum scale for the camera
    ///
    /// The orthographic projection's scale will be clamped at this value when
    /// zooming in. Pass `f32::NEG_INFINITY` to disable clamping.
    pub min_scale: f32,
    /// The maximum scale for the camera
    ///
    /// The orthographic projection's scale will be clamped at this value when
    /// zooming out. Pass `f32::INFINITY` to disable clamping.
    pub max_scale: f32,
    /// The minimum x position of the camera window
    ///
    /// The orthographic projection will be clamped to this boundary both when
    /// dragging the window, and zooming out. Pass `f32::NEG_INFINITY` to disable
    /// clamping.
    pub min_x: f32,
    /// The maximum x position of the camera window
    ///
    /// The orthographic projection will be clamped to this boundary both when
    /// dragging the window, and zooming out. Pass `f32::INFINITY` to disable
    /// clamping.
    pub max_x: f32,
    /// The minimum y position of the camera window
    ///
    /// The orthographic projection will be clamped to this boundary both when
    /// dragging the window, and zooming out. Pass `f32::NEG_INFINITY` to disable
    /// clamping.
    pub min_y: f32,
    /// The maximum y position of the camera window
    ///
    /// The orthographic projection will be clamped to this boundary both when
    /// dragging the window, and zooming out. Pass `f32::INFINITY` to disable
    /// clamping.
    pub max_y: f32,
    /// 触摸板缩放比例系数
    pub touch_pad_zoom_scale: f32,
    /// 鼠标滚轮滚动平移缩放系数
    pub mouse_wheel_pan_scale: f32,
    pub touch_config: TouchCameraConfig,
}

impl PanCam {
    /// Returns (min, max) bound tuple
    fn bounds(&self) -> (Vec2, Vec2) {
        let min = vec2(self.min_x, self.min_y);
        let max = vec2(self.max_x, self.max_y);
        (min, max)
    }

    /// Returns the bounding `Rect`
    fn rect(&self) -> Rect {
        let (min, max) = self.bounds();
        Rect { min, max }
    }

    /// Returns the bounding `Aabb2d`
    fn aabb(&self) -> Aabb2d {
        let (min, max) = self.bounds();
        Aabb2d { min, max }
    }

    /// Returns the scale inclusive range
    fn scale_range(&self) -> RangeInclusive<f32> {
        self.min_scale..=self.max_scale
    }
}

impl Default for PanCam {
    fn default() -> Self {
        Self {
            move_keys: DirectionKeys::arrows_and_wasd(),
            speed: 200.,
            grab_buttons: vec![MouseButton::Left, MouseButton::Right, MouseButton::Middle],
            enabled: true,
            zoom_to_cursor: true,
            min_scale: 0.00001,
            max_scale: 100.,
            min_x: f32::NEG_INFINITY,
            max_x: f32::INFINITY,
            min_y: f32::NEG_INFINITY,
            max_y: f32::INFINITY,
            touch_pad_zoom_scale: 10.0,
            mouse_wheel_pan_scale: 0.3,
            touch_config: Default::default(),
        }
    }
}

/// Contains the configuration parameters for the plugin.
/// A copy of this will be attached as a `Resource` to the `App`.
#[derive(Resource, Clone, Reflect)]
pub struct TouchCameraConfig {
    /// How far the camera will move relative to the touch drag distance. Higher is faster
    pub drag_sensitivity: f32,
    /// How much the camera will zoom relative to the pinch distance using two fingers. Higher means faster.
    /// At the moment the default is very low at 0.005 but this might change in the future
    pub zoom_sensitivity: f32,
    /// Minimum time before starting to pan in seconds. Useful to avoid panning on short taps
    pub touch_time_min: f32,
    /// Tolerance for pinch fingers moving in opposite directions (-1. .. 1.).
    /// Higher values make it more tolerant.
    /// Very low values not recommended as it would be overly sensitive
    pub opposites_tolerance: f32,
}

impl Default for TouchCameraConfig {
    fn default() -> Self {
        Self {
            drag_sensitivity: 1.,
            zoom_sensitivity: 0.01,
            touch_time_min: 0.01,
            opposites_tolerance: 0.,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32::INFINITY;

    use bevy::prelude::OrthographicProjection;

    use super::*;

    /// Simple mock function to construct a square projection from a window size
    fn mock_proj(window_size: Vec2) -> OrthographicProjection {
        let mut proj = Camera2dBundle::default().projection;
        proj.update(window_size.x, window_size.y);
        proj
    }

    #[test]
    fn bounds_matching_window_width_have_max_scale_1() {
        let window_size = vec2(100., 100.);
        let proj = mock_proj(window_size);
        assert_eq!(
            max_scale_within_bounds(vec2(100., INFINITY), &proj, window_size).x,
            1.
        );
    }

    // boundaries are 1/2 the size of the projection window
    #[test]
    fn bounds_half_of_window_width_have_half_max_scale() {
        let window_size = vec2(100., 100.);
        let proj = mock_proj(window_size);
        assert_eq!(
            max_scale_within_bounds(vec2(50., INFINITY), &proj, window_size).x,
            0.5
        );
    }

    // boundaries are 2x the size of the projection window
    #[test]
    fn bounds_twice_of_window_width_have_max_scale_2() {
        let window_size = vec2(100., 100.);
        let proj = mock_proj(window_size);
        assert_eq!(
            max_scale_within_bounds(vec2(200., INFINITY), &proj, window_size).x,
            2.
        );
    }

    #[test]
    fn bounds_matching_window_height_have_max_scale_1() {
        let window_size = vec2(100., 100.);
        let proj = mock_proj(window_size);
        assert_eq!(
            max_scale_within_bounds(vec2(INFINITY, 100.), &proj, window_size).y,
            1.
        );
    }

    // boundaries are 1/2 the size of the projection window
    #[test]
    fn bounds_half_of_window_height_have_half_max_scale() {
        let window_size = vec2(100., 100.);
        let proj = mock_proj(window_size);
        assert_eq!(
            max_scale_within_bounds(vec2(INFINITY, 50.), &proj, window_size).y,
            0.5
        );
    }

    // boundaries are 2x the size of the projection window
    #[test]
    fn bounds_twice_of_window_height_have_max_scale_2() {
        let window_size = vec2(100., 100.);
        let proj = mock_proj(window_size);
        assert_eq!(
            max_scale_within_bounds(vec2(INFINITY, 200.), &proj, window_size).y,
            2.
        );
    }
}
