#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
pub const RL_DEFAULT_BATCH_BUFFER_ELEMENTS: u32 = 8192;
pub const RL_DEFAULT_BATCH_BUFFERS: u32 = 1;
pub const RL_DEFAULT_BATCH_DRAWCALLS: u32 = 256;
pub const RL_DEFAULT_BATCH_MAX_TEXTURE_UNITS: u32 = 4;
pub const RL_MAX_MATRIX_STACK_SIZE: u32 = 32;
pub const RL_MAX_SHADER_LOCATIONS: u32 = 32;
pub const RL_CULL_DISTANCE_NEAR: f64 = 0.01;
pub const RL_CULL_DISTANCE_FAR: f64 = 1000.0;
pub const RL_TEXTURE_WRAP_S: u32 = 10242;
pub const RL_TEXTURE_WRAP_T: u32 = 10243;
pub const RL_TEXTURE_MAG_FILTER: u32 = 10240;
pub const RL_TEXTURE_MIN_FILTER: u32 = 10241;
pub const RL_TEXTURE_FILTER_NEAREST: u32 = 9728;
pub const RL_TEXTURE_FILTER_LINEAR: u32 = 9729;
pub const RL_TEXTURE_FILTER_MIP_NEAREST: u32 = 9984;
pub const RL_TEXTURE_FILTER_NEAREST_MIP_LINEAR: u32 = 9986;
pub const RL_TEXTURE_FILTER_LINEAR_MIP_NEAREST: u32 = 9985;
pub const RL_TEXTURE_FILTER_MIP_LINEAR: u32 = 9987;
pub const RL_TEXTURE_FILTER_ANISOTROPIC: u32 = 12288;
pub const RL_TEXTURE_MIPMAP_BIAS_RATIO: u32 = 16384;
pub const RL_TEXTURE_WRAP_REPEAT: u32 = 10497;
pub const RL_TEXTURE_WRAP_CLAMP: u32 = 33071;
pub const RL_TEXTURE_WRAP_MIRROR_REPEAT: u32 = 33648;
pub const RL_TEXTURE_WRAP_MIRROR_CLAMP: u32 = 34626;
pub const RL_MODELVIEW: u32 = 5888;
pub const RL_PROJECTION: u32 = 5889;
pub const RL_TEXTURE: u32 = 5890;
pub const RL_LINES: u32 = 1;
pub const RL_TRIANGLES: u32 = 4;
pub const RL_QUADS: u32 = 7;
pub const RL_UNSIGNED_BYTE: u32 = 5121;
pub const RL_FLOAT: u32 = 5126;
pub const RL_STREAM_DRAW: u32 = 35040;
pub const RL_STREAM_READ: u32 = 35041;
pub const RL_STREAM_COPY: u32 = 35042;
pub const RL_STATIC_DRAW: u32 = 35044;
pub const RL_STATIC_READ: u32 = 35045;
pub const RL_STATIC_COPY: u32 = 35046;
pub const RL_DYNAMIC_DRAW: u32 = 35048;
pub const RL_DYNAMIC_READ: u32 = 35049;
pub const RL_DYNAMIC_COPY: u32 = 35050;
pub const RL_FRAGMENT_SHADER: u32 = 35632;
pub const RL_VERTEX_SHADER: u32 = 35633;
pub const RL_COMPUTE_SHADER: u32 = 37305;
pub const RL_SRC_COLOR: u32 = 768;
pub const RL_ONE_MINUS_SRC_COLOR: u32 = 769;
pub const RL_SRC_ALPHA: u32 = 770;
pub const RL_ONE_MINUS_SRC_ALPHA: u32 = 771;
pub const RL_DST_ALPHA: u32 = 772;
pub const RL_ONE_MINUS_DST_ALPHA: u32 = 773;
pub const RL_DST_COLOR: u32 = 774;
pub const RL_ONE_MINUS_DST_COLOR: u32 = 775;
pub const RL_SRC_ALPHA_SATURATE: u32 = 776;
pub const RL_CONSTANT_COLOR: u32 = 32769;
pub const RL_ONE_MINUS_CONSTANT_COLOR: u32 = 32770;
pub const RL_CONSTANT_ALPHA: u32 = 32771;
pub const RL_ONE_MINUS_CONSTANT_ALPHA: u32 = 32772;
pub const RL_FUNC_ADD: u32 = 32774;
pub const RL_FUNC_SUBTRACT: u32 = 32778;
pub const RL_FUNC_REVERSE_SUBTRACT: u32 = 32779;
pub const RL_BLEND_EQUATION: u32 = 32777;
pub const RL_BLEND_EQUATION_RGB: u32 = 32777;
pub const RL_BLEND_EQUATION_ALPHA: u32 = 34877;
pub const RL_BLEND_DST_RGB: u32 = 32968;
pub const RL_BLEND_SRC_RGB: u32 = 32969;
pub const RL_BLEND_DST_ALPHA: u32 = 32970;
pub const RL_BLEND_SRC_ALPHA: u32 = 32971;
pub const RL_BLEND_COLOR: u32 = 32773;
pub const RL_READ_FRAMEBUFFER: u32 = 36008;
pub const RL_DRAW_FRAMEBUFFER: u32 = 36009;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_POSITION: u32 = 0;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_TEXCOORD: u32 = 1;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_NORMAL: u32 = 2;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_COLOR: u32 = 3;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_TANGENT: u32 = 4;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_TEXCOORD2: u32 = 5;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_INDICES: u32 = 6;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
pub type Quaternion = Vector4;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Matrix {
    pub m0: f32,
    pub m4: f32,
    pub m8: f32,
    pub m12: f32,
    pub m1: f32,
    pub m5: f32,
    pub m9: f32,
    pub m13: f32,
    pub m2: f32,
    pub m6: f32,
    pub m10: f32,
    pub m14: f32,
    pub m3: f32,
    pub m7: f32,
    pub m11: f32,
    pub m15: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: ::std::os::raw::c_uchar,
    pub g: ::std::os::raw::c_uchar,
    pub b: ::std::os::raw::c_uchar,
    pub a: ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Image {
    pub data: *mut ::std::os::raw::c_void,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub mipmaps: ::std::os::raw::c_int,
    pub format: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Texture {
    pub id: ::std::os::raw::c_uint,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub mipmaps: ::std::os::raw::c_int,
    pub format: ::std::os::raw::c_int,
}
pub type Texture2D = Texture;
pub type TextureCubemap = Texture;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RenderTexture {
    pub id: ::std::os::raw::c_uint,
    pub texture: Texture,
    pub depth: Texture,
}
pub type RenderTexture2D = RenderTexture;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NPatchInfo {
    pub source: Rectangle,
    pub left: ::std::os::raw::c_int,
    pub top: ::std::os::raw::c_int,
    pub right: ::std::os::raw::c_int,
    pub bottom: ::std::os::raw::c_int,
    pub layout: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GlyphInfo {
    pub value: ::std::os::raw::c_int,
    pub offsetX: ::std::os::raw::c_int,
    pub offsetY: ::std::os::raw::c_int,
    pub advanceX: ::std::os::raw::c_int,
    pub image: Image,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Font {
    pub baseSize: ::std::os::raw::c_int,
    pub glyphCount: ::std::os::raw::c_int,
    pub glyphPadding: ::std::os::raw::c_int,
    pub texture: Texture2D,
    pub recs: *mut Rectangle,
    pub glyphs: *mut GlyphInfo,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera3D {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub fovy: f32,
    pub projection: ::std::os::raw::c_int,
}
pub type Camera = Camera3D;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera2D {
    pub offset: Vector2,
    pub target: Vector2,
    pub rotation: f32,
    pub zoom: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mesh {
    pub vertexCount: ::std::os::raw::c_int,
    pub triangleCount: ::std::os::raw::c_int,
    pub vertices: *mut f32,
    pub texcoords: *mut f32,
    pub texcoords2: *mut f32,
    pub normals: *mut f32,
    pub tangents: *mut f32,
    pub colors: *mut ::std::os::raw::c_uchar,
    pub indices: *mut ::std::os::raw::c_ushort,
    pub animVertices: *mut f32,
    pub animNormals: *mut f32,
    pub boneIds: *mut ::std::os::raw::c_uchar,
    pub boneWeights: *mut f32,
    pub boneMatrices: *mut Matrix,
    pub boneCount: ::std::os::raw::c_int,
    pub vaoId: ::std::os::raw::c_uint,
    pub vboId: *mut ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Shader {
    pub id: ::std::os::raw::c_uint,
    pub locs: *mut ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MaterialMap {
    pub texture: Texture2D,
    pub color: Color,
    pub value: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub shader: Shader,
    pub maps: *mut MaterialMap,
    pub params: [f32; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub translation: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BoneInfo {
    pub name: [::std::os::raw::c_char; 32usize],
    pub parent: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Model {
    pub transform: Matrix,
    pub meshCount: ::std::os::raw::c_int,
    pub materialCount: ::std::os::raw::c_int,
    pub meshes: *mut Mesh,
    pub materials: *mut Material,
    pub meshMaterial: *mut ::std::os::raw::c_int,
    pub boneCount: ::std::os::raw::c_int,
    pub bones: *mut BoneInfo,
    pub bindPose: *mut Transform,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModelAnimation {
    pub boneCount: ::std::os::raw::c_int,
    pub frameCount: ::std::os::raw::c_int,
    pub bones: *mut BoneInfo,
    pub framePoses: *mut *mut Transform,
    pub name: [::std::os::raw::c_char; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub position: Vector3,
    pub direction: Vector3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RayCollision {
    pub hit: bool,
    pub distance: f32,
    pub point: Vector3,
    pub normal: Vector3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    pub min: Vector3,
    pub max: Vector3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Wave {
    pub frameCount: ::std::os::raw::c_uint,
    pub sampleRate: ::std::os::raw::c_uint,
    pub sampleSize: ::std::os::raw::c_uint,
    pub channels: ::std::os::raw::c_uint,
    pub data: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rAudioBuffer {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rAudioProcessor {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioStream {
    pub buffer: *mut rAudioBuffer,
    pub processor: *mut rAudioProcessor,
    pub sampleRate: ::std::os::raw::c_uint,
    pub sampleSize: ::std::os::raw::c_uint,
    pub channels: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Sound {
    pub stream: AudioStream,
    pub frameCount: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Music {
    pub stream: AudioStream,
    pub frameCount: ::std::os::raw::c_uint,
    pub looping: bool,
    pub ctxType: ::std::os::raw::c_int,
    pub ctxData: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VrDeviceInfo {
    pub hResolution: ::std::os::raw::c_int,
    pub vResolution: ::std::os::raw::c_int,
    pub hScreenSize: f32,
    pub vScreenSize: f32,
    pub eyeToScreenDistance: f32,
    pub lensSeparationDistance: f32,
    pub interpupillaryDistance: f32,
    pub lensDistortionValues: [f32; 4usize],
    pub chromaAbCorrection: [f32; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VrStereoConfig {
    pub projection: [Matrix; 2usize],
    pub viewOffset: [Matrix; 2usize],
    pub leftLensCenter: [f32; 2usize],
    pub rightLensCenter: [f32; 2usize],
    pub leftScreenCenter: [f32; 2usize],
    pub rightScreenCenter: [f32; 2usize],
    pub scale: [f32; 2usize],
    pub scaleIn: [f32; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FilePathList {
    pub capacity: ::std::os::raw::c_uint,
    pub count: ::std::os::raw::c_uint,
    pub paths: *mut *mut ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AutomationEvent {
    pub frame: ::std::os::raw::c_uint,
    pub type_: ::std::os::raw::c_uint,
    pub params: [::std::os::raw::c_int; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AutomationEventList {
    pub capacity: ::std::os::raw::c_uint,
    pub count: ::std::os::raw::c_uint,
    pub events: *mut AutomationEvent,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ConfigFlags {
    FLAG_VSYNC_HINT = 64,
    FLAG_FULLSCREEN_MODE = 2,
    FLAG_WINDOW_RESIZABLE = 4,
    FLAG_WINDOW_UNDECORATED = 8,
    FLAG_WINDOW_HIDDEN = 128,
    FLAG_WINDOW_MINIMIZED = 512,
    FLAG_WINDOW_MAXIMIZED = 1024,
    FLAG_WINDOW_UNFOCUSED = 2048,
    FLAG_WINDOW_TOPMOST = 4096,
    FLAG_WINDOW_ALWAYS_RUN = 256,
    FLAG_WINDOW_TRANSPARENT = 16,
    FLAG_WINDOW_HIGHDPI = 8192,
    FLAG_WINDOW_MOUSE_PASSTHROUGH = 16384,
    FLAG_BORDERLESS_WINDOWED_MODE = 32768,
    FLAG_MSAA_4X_HINT = 32,
    FLAG_INTERLACED_HINT = 65536,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TraceLogLevel {
    LOG_ALL = 0,
    LOG_TRACE = 1,
    LOG_DEBUG = 2,
    LOG_INFO = 3,
    LOG_WARNING = 4,
    LOG_ERROR = 5,
    LOG_FATAL = 6,
    LOG_NONE = 7,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KeyboardKey {
    KEY_NULL = 0,
    KEY_APOSTROPHE = 39,
    KEY_COMMA = 44,
    KEY_MINUS = 45,
    KEY_PERIOD = 46,
    KEY_SLASH = 47,
    KEY_ZERO = 48,
    KEY_ONE = 49,
    KEY_TWO = 50,
    KEY_THREE = 51,
    KEY_FOUR = 52,
    KEY_FIVE = 53,
    KEY_SIX = 54,
    KEY_SEVEN = 55,
    KEY_EIGHT = 56,
    KEY_NINE = 57,
    KEY_SEMICOLON = 59,
    KEY_EQUAL = 61,
    KEY_A = 65,
    KEY_B = 66,
    KEY_C = 67,
    KEY_D = 68,
    KEY_E = 69,
    KEY_F = 70,
    KEY_G = 71,
    KEY_H = 72,
    KEY_I = 73,
    KEY_J = 74,
    KEY_K = 75,
    KEY_L = 76,
    KEY_M = 77,
    KEY_N = 78,
    KEY_O = 79,
    KEY_P = 80,
    KEY_Q = 81,
    KEY_R = 82,
    KEY_S = 83,
    KEY_T = 84,
    KEY_U = 85,
    KEY_V = 86,
    KEY_W = 87,
    KEY_X = 88,
    KEY_Y = 89,
    KEY_Z = 90,
    KEY_LEFT_BRACKET = 91,
    KEY_BACKSLASH = 92,
    KEY_RIGHT_BRACKET = 93,
    KEY_GRAVE = 96,
    KEY_SPACE = 32,
    KEY_ESCAPE = 256,
    KEY_ENTER = 257,
    KEY_TAB = 258,
    KEY_BACKSPACE = 259,
    KEY_INSERT = 260,
    KEY_DELETE = 261,
    KEY_RIGHT = 262,
    KEY_LEFT = 263,
    KEY_DOWN = 264,
    KEY_UP = 265,
    KEY_PAGE_UP = 266,
    KEY_PAGE_DOWN = 267,
    KEY_HOME = 268,
    KEY_END = 269,
    KEY_CAPS_LOCK = 280,
    KEY_SCROLL_LOCK = 281,
    KEY_NUM_LOCK = 282,
    KEY_PRINT_SCREEN = 283,
    KEY_PAUSE = 284,
    KEY_F1 = 290,
    KEY_F2 = 291,
    KEY_F3 = 292,
    KEY_F4 = 293,
    KEY_F5 = 294,
    KEY_F6 = 295,
    KEY_F7 = 296,
    KEY_F8 = 297,
    KEY_F9 = 298,
    KEY_F10 = 299,
    KEY_F11 = 300,
    KEY_F12 = 301,
    KEY_LEFT_SHIFT = 340,
    KEY_LEFT_CONTROL = 341,
    KEY_LEFT_ALT = 342,
    KEY_LEFT_SUPER = 343,
    KEY_RIGHT_SHIFT = 344,
    KEY_RIGHT_CONTROL = 345,
    KEY_RIGHT_ALT = 346,
    KEY_RIGHT_SUPER = 347,
    KEY_KB_MENU = 348,
    KEY_KP_0 = 320,
    KEY_KP_1 = 321,
    KEY_KP_2 = 322,
    KEY_KP_3 = 323,
    KEY_KP_4 = 324,
    KEY_KP_5 = 325,
    KEY_KP_6 = 326,
    KEY_KP_7 = 327,
    KEY_KP_8 = 328,
    KEY_KP_9 = 329,
    KEY_KP_DECIMAL = 330,
    KEY_KP_DIVIDE = 331,
    KEY_KP_MULTIPLY = 332,
    KEY_KP_SUBTRACT = 333,
    KEY_KP_ADD = 334,
    KEY_KP_ENTER = 335,
    KEY_KP_EQUAL = 336,
    KEY_BACK = 4,
    KEY_MENU = 5,
    KEY_VOLUME_UP = 24,
    KEY_VOLUME_DOWN = 25,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MouseButton {
    MOUSE_BUTTON_LEFT = 0,
    MOUSE_BUTTON_RIGHT = 1,
    MOUSE_BUTTON_MIDDLE = 2,
    MOUSE_BUTTON_SIDE = 3,
    MOUSE_BUTTON_EXTRA = 4,
    MOUSE_BUTTON_FORWARD = 5,
    MOUSE_BUTTON_BACK = 6,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MouseCursor {
    MOUSE_CURSOR_DEFAULT = 0,
    MOUSE_CURSOR_ARROW = 1,
    MOUSE_CURSOR_IBEAM = 2,
    MOUSE_CURSOR_CROSSHAIR = 3,
    MOUSE_CURSOR_POINTING_HAND = 4,
    MOUSE_CURSOR_RESIZE_EW = 5,
    MOUSE_CURSOR_RESIZE_NS = 6,
    MOUSE_CURSOR_RESIZE_NWSE = 7,
    MOUSE_CURSOR_RESIZE_NESW = 8,
    MOUSE_CURSOR_RESIZE_ALL = 9,
    MOUSE_CURSOR_NOT_ALLOWED = 10,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GamepadButton {
    GAMEPAD_BUTTON_UNKNOWN = 0,
    GAMEPAD_BUTTON_LEFT_FACE_UP = 1,
    GAMEPAD_BUTTON_LEFT_FACE_RIGHT = 2,
    GAMEPAD_BUTTON_LEFT_FACE_DOWN = 3,
    GAMEPAD_BUTTON_LEFT_FACE_LEFT = 4,
    GAMEPAD_BUTTON_RIGHT_FACE_UP = 5,
    GAMEPAD_BUTTON_RIGHT_FACE_RIGHT = 6,
    GAMEPAD_BUTTON_RIGHT_FACE_DOWN = 7,
    GAMEPAD_BUTTON_RIGHT_FACE_LEFT = 8,
    GAMEPAD_BUTTON_LEFT_TRIGGER_1 = 9,
    GAMEPAD_BUTTON_LEFT_TRIGGER_2 = 10,
    GAMEPAD_BUTTON_RIGHT_TRIGGER_1 = 11,
    GAMEPAD_BUTTON_RIGHT_TRIGGER_2 = 12,
    GAMEPAD_BUTTON_MIDDLE_LEFT = 13,
    GAMEPAD_BUTTON_MIDDLE = 14,
    GAMEPAD_BUTTON_MIDDLE_RIGHT = 15,
    GAMEPAD_BUTTON_LEFT_THUMB = 16,
    GAMEPAD_BUTTON_RIGHT_THUMB = 17,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GamepadAxis {
    GAMEPAD_AXIS_LEFT_X = 0,
    GAMEPAD_AXIS_LEFT_Y = 1,
    GAMEPAD_AXIS_RIGHT_X = 2,
    GAMEPAD_AXIS_RIGHT_Y = 3,
    GAMEPAD_AXIS_LEFT_TRIGGER = 4,
    GAMEPAD_AXIS_RIGHT_TRIGGER = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MaterialMapIndex {
    MATERIAL_MAP_ALBEDO = 0,
    MATERIAL_MAP_METALNESS = 1,
    MATERIAL_MAP_NORMAL = 2,
    MATERIAL_MAP_ROUGHNESS = 3,
    MATERIAL_MAP_OCCLUSION = 4,
    MATERIAL_MAP_EMISSION = 5,
    MATERIAL_MAP_HEIGHT = 6,
    MATERIAL_MAP_CUBEMAP = 7,
    MATERIAL_MAP_IRRADIANCE = 8,
    MATERIAL_MAP_PREFILTER = 9,
    MATERIAL_MAP_BRDF = 10,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ShaderLocationIndex {
    SHADER_LOC_VERTEX_POSITION = 0,
    SHADER_LOC_VERTEX_TEXCOORD01 = 1,
    SHADER_LOC_VERTEX_TEXCOORD02 = 2,
    SHADER_LOC_VERTEX_NORMAL = 3,
    SHADER_LOC_VERTEX_TANGENT = 4,
    SHADER_LOC_VERTEX_COLOR = 5,
    SHADER_LOC_MATRIX_MVP = 6,
    SHADER_LOC_MATRIX_VIEW = 7,
    SHADER_LOC_MATRIX_PROJECTION = 8,
    SHADER_LOC_MATRIX_MODEL = 9,
    SHADER_LOC_MATRIX_NORMAL = 10,
    SHADER_LOC_VECTOR_VIEW = 11,
    SHADER_LOC_COLOR_DIFFUSE = 12,
    SHADER_LOC_COLOR_SPECULAR = 13,
    SHADER_LOC_COLOR_AMBIENT = 14,
    SHADER_LOC_MAP_ALBEDO = 15,
    SHADER_LOC_MAP_METALNESS = 16,
    SHADER_LOC_MAP_NORMAL = 17,
    SHADER_LOC_MAP_ROUGHNESS = 18,
    SHADER_LOC_MAP_OCCLUSION = 19,
    SHADER_LOC_MAP_EMISSION = 20,
    SHADER_LOC_MAP_HEIGHT = 21,
    SHADER_LOC_MAP_CUBEMAP = 22,
    SHADER_LOC_MAP_IRRADIANCE = 23,
    SHADER_LOC_MAP_PREFILTER = 24,
    SHADER_LOC_MAP_BRDF = 25,
    SHADER_LOC_VERTEX_BONEIDS = 26,
    SHADER_LOC_VERTEX_BONEWEIGHTS = 27,
    SHADER_LOC_BONE_MATRICES = 28,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ShaderUniformDataType {
    SHADER_UNIFORM_FLOAT = 0,
    SHADER_UNIFORM_VEC2 = 1,
    SHADER_UNIFORM_VEC3 = 2,
    SHADER_UNIFORM_VEC4 = 3,
    SHADER_UNIFORM_INT = 4,
    SHADER_UNIFORM_IVEC2 = 5,
    SHADER_UNIFORM_IVEC3 = 6,
    SHADER_UNIFORM_IVEC4 = 7,
    SHADER_UNIFORM_SAMPLER2D = 8,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ShaderAttributeDataType {
    SHADER_ATTRIB_FLOAT = 0,
    SHADER_ATTRIB_VEC2 = 1,
    SHADER_ATTRIB_VEC3 = 2,
    SHADER_ATTRIB_VEC4 = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PixelFormat {
    PIXELFORMAT_UNCOMPRESSED_GRAYSCALE = 1,
    PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA = 2,
    PIXELFORMAT_UNCOMPRESSED_R5G6B5 = 3,
    PIXELFORMAT_UNCOMPRESSED_R8G8B8 = 4,
    PIXELFORMAT_UNCOMPRESSED_R5G5B5A1 = 5,
    PIXELFORMAT_UNCOMPRESSED_R4G4B4A4 = 6,
    PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 = 7,
    PIXELFORMAT_UNCOMPRESSED_R32 = 8,
    PIXELFORMAT_UNCOMPRESSED_R32G32B32 = 9,
    PIXELFORMAT_UNCOMPRESSED_R32G32B32A32 = 10,
    PIXELFORMAT_UNCOMPRESSED_R16 = 11,
    PIXELFORMAT_UNCOMPRESSED_R16G16B16 = 12,
    PIXELFORMAT_UNCOMPRESSED_R16G16B16A16 = 13,
    PIXELFORMAT_COMPRESSED_DXT1_RGB = 14,
    PIXELFORMAT_COMPRESSED_DXT1_RGBA = 15,
    PIXELFORMAT_COMPRESSED_DXT3_RGBA = 16,
    PIXELFORMAT_COMPRESSED_DXT5_RGBA = 17,
    PIXELFORMAT_COMPRESSED_ETC1_RGB = 18,
    PIXELFORMAT_COMPRESSED_ETC2_RGB = 19,
    PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA = 20,
    PIXELFORMAT_COMPRESSED_PVRT_RGB = 21,
    PIXELFORMAT_COMPRESSED_PVRT_RGBA = 22,
    PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA = 23,
    PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA = 24,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TextureFilter {
    TEXTURE_FILTER_POINT = 0,
    TEXTURE_FILTER_BILINEAR = 1,
    TEXTURE_FILTER_TRILINEAR = 2,
    TEXTURE_FILTER_ANISOTROPIC_4X = 3,
    TEXTURE_FILTER_ANISOTROPIC_8X = 4,
    TEXTURE_FILTER_ANISOTROPIC_16X = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TextureWrap {
    TEXTURE_WRAP_REPEAT = 0,
    TEXTURE_WRAP_CLAMP = 1,
    TEXTURE_WRAP_MIRROR_REPEAT = 2,
    TEXTURE_WRAP_MIRROR_CLAMP = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CubemapLayout {
    CUBEMAP_LAYOUT_AUTO_DETECT = 0,
    CUBEMAP_LAYOUT_LINE_VERTICAL = 1,
    CUBEMAP_LAYOUT_LINE_HORIZONTAL = 2,
    CUBEMAP_LAYOUT_CROSS_THREE_BY_FOUR = 3,
    CUBEMAP_LAYOUT_CROSS_FOUR_BY_THREE = 4,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FontType {
    FONT_DEFAULT = 0,
    FONT_BITMAP = 1,
    FONT_SDF = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BlendMode {
    BLEND_ALPHA = 0,
    BLEND_ADDITIVE = 1,
    BLEND_MULTIPLIED = 2,
    BLEND_ADD_COLORS = 3,
    BLEND_SUBTRACT_COLORS = 4,
    BLEND_ALPHA_PREMULTIPLY = 5,
    BLEND_CUSTOM = 6,
    BLEND_CUSTOM_SEPARATE = 7,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Gesture {
    GESTURE_NONE = 0,
    GESTURE_TAP = 1,
    GESTURE_DOUBLETAP = 2,
    GESTURE_HOLD = 4,
    GESTURE_DRAG = 8,
    GESTURE_SWIPE_RIGHT = 16,
    GESTURE_SWIPE_LEFT = 32,
    GESTURE_SWIPE_UP = 64,
    GESTURE_SWIPE_DOWN = 128,
    GESTURE_PINCH_IN = 256,
    GESTURE_PINCH_OUT = 512,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CameraMode {
    CAMERA_CUSTOM = 0,
    CAMERA_FREE = 1,
    CAMERA_ORBITAL = 2,
    CAMERA_FIRST_PERSON = 3,
    CAMERA_THIRD_PERSON = 4,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CameraProjection {
    CAMERA_PERSPECTIVE = 0,
    CAMERA_ORTHOGRAPHIC = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum NPatchLayout {
    NPATCH_NINE_PATCH = 0,
    NPATCH_THREE_PATCH_VERTICAL = 1,
    NPATCH_THREE_PATCH_HORIZONTAL = 2,
}
pub type TraceLogCallback = ::std::option::Option<
    unsafe extern "C" fn(
        logLevel: ::std::os::raw::c_int,
        text: *const ::std::os::raw::c_char,
        args: *mut __va_list_tag,
    ),
>;
pub type LoadFileDataCallback = ::std::option::Option<
    unsafe extern "C" fn(
        fileName: *const ::std::os::raw::c_char,
        dataSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar,
>;
pub type SaveFileDataCallback = ::std::option::Option<
    unsafe extern "C" fn(
        fileName: *const ::std::os::raw::c_char,
        data: *mut ::std::os::raw::c_void,
        dataSize: ::std::os::raw::c_int,
    ) -> bool,
>;
pub type LoadFileTextCallback = ::std::option::Option<
    unsafe extern "C" fn(fileName: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char,
>;
pub type SaveFileTextCallback = ::std::option::Option<
    unsafe extern "C" fn(
        fileName: *const ::std::os::raw::c_char,
        text: *mut ::std::os::raw::c_char,
    ) -> bool,
>;
unsafe extern "C" {
    pub fn InitWindow(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn CloseWindow();
}
unsafe extern "C" {
    pub fn WindowShouldClose() -> bool;
}
unsafe extern "C" {
    pub fn IsWindowReady() -> bool;
}
unsafe extern "C" {
    pub fn IsWindowFullscreen() -> bool;
}
unsafe extern "C" {
    pub fn IsWindowHidden() -> bool;
}
unsafe extern "C" {
    pub fn IsWindowMinimized() -> bool;
}
unsafe extern "C" {
    pub fn IsWindowMaximized() -> bool;
}
unsafe extern "C" {
    pub fn IsWindowFocused() -> bool;
}
unsafe extern "C" {
    pub fn IsWindowResized() -> bool;
}
unsafe extern "C" {
    pub fn IsWindowState(flag: ::std::os::raw::c_uint) -> bool;
}
unsafe extern "C" {
    pub fn SetWindowState(flags: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn ClearWindowState(flags: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn ToggleFullscreen();
}
unsafe extern "C" {
    pub fn ToggleBorderlessWindowed();
}
unsafe extern "C" {
    pub fn MaximizeWindow();
}
unsafe extern "C" {
    pub fn MinimizeWindow();
}
unsafe extern "C" {
    pub fn RestoreWindow();
}
unsafe extern "C" {
    pub fn SetWindowIcon(image: Image);
}
unsafe extern "C" {
    pub fn SetWindowIcons(images: *mut Image, count: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn SetWindowTitle(title: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn SetWindowPosition(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn SetWindowMonitor(monitor: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn SetWindowMinSize(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn SetWindowMaxSize(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn SetWindowSize(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn SetWindowOpacity(opacity: f32);
}
unsafe extern "C" {
    pub fn SetWindowFocused();
}
unsafe extern "C" {
    pub fn GetWindowHandle() -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn GetScreenWidth() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetScreenHeight() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetRenderWidth() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetRenderHeight() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetMonitorCount() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetCurrentMonitor() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetMonitorPosition(monitor: ::std::os::raw::c_int) -> Vector2;
}
unsafe extern "C" {
    pub fn GetMonitorWidth(monitor: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetMonitorHeight(monitor: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetMonitorPhysicalWidth(monitor: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetMonitorPhysicalHeight(monitor: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetMonitorRefreshRate(monitor: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetWindowPosition() -> Vector2;
}
unsafe extern "C" {
    pub fn GetWindowScaleDPI() -> Vector2;
}
unsafe extern "C" {
    pub fn GetMonitorName(monitor: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn SetClipboardText(text: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn GetClipboardText() -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn GetClipboardImage() -> Image;
}
unsafe extern "C" {
    pub fn EnableEventWaiting();
}
unsafe extern "C" {
    pub fn DisableEventWaiting();
}
unsafe extern "C" {
    pub fn ShowCursor();
}
unsafe extern "C" {
    pub fn HideCursor();
}
unsafe extern "C" {
    pub fn IsCursorHidden() -> bool;
}
unsafe extern "C" {
    pub fn EnableCursor();
}
unsafe extern "C" {
    pub fn DisableCursor();
}
unsafe extern "C" {
    pub fn IsCursorOnScreen() -> bool;
}
unsafe extern "C" {
    pub fn ClearBackground(color: Color);
}
unsafe extern "C" {
    pub fn BeginDrawing();
}
unsafe extern "C" {
    pub fn EndDrawing();
}
unsafe extern "C" {
    pub fn BeginMode2D(camera: Camera2D);
}
unsafe extern "C" {
    pub fn EndMode2D();
}
unsafe extern "C" {
    pub fn BeginMode3D(camera: Camera3D);
}
unsafe extern "C" {
    pub fn EndMode3D();
}
unsafe extern "C" {
    pub fn BeginTextureMode(target: RenderTexture2D);
}
unsafe extern "C" {
    pub fn EndTextureMode();
}
unsafe extern "C" {
    pub fn BeginShaderMode(shader: Shader);
}
unsafe extern "C" {
    pub fn EndShaderMode();
}
unsafe extern "C" {
    pub fn BeginBlendMode(mode: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn EndBlendMode();
}
unsafe extern "C" {
    pub fn BeginScissorMode(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn EndScissorMode();
}
unsafe extern "C" {
    pub fn BeginVrStereoMode(config: VrStereoConfig);
}
unsafe extern "C" {
    pub fn EndVrStereoMode();
}
unsafe extern "C" {
    pub fn LoadVrStereoConfig(device: VrDeviceInfo) -> VrStereoConfig;
}
unsafe extern "C" {
    pub fn UnloadVrStereoConfig(config: VrStereoConfig);
}
unsafe extern "C" {
    pub fn LoadShader(
        vsFileName: *const ::std::os::raw::c_char,
        fsFileName: *const ::std::os::raw::c_char,
    ) -> Shader;
}
unsafe extern "C" {
    pub fn LoadShaderFromMemory(
        vsCode: *const ::std::os::raw::c_char,
        fsCode: *const ::std::os::raw::c_char,
    ) -> Shader;
}
unsafe extern "C" {
    pub fn IsShaderValid(shader: Shader) -> bool;
}
unsafe extern "C" {
    pub fn GetShaderLocation(
        shader: Shader,
        uniformName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetShaderLocationAttrib(
        shader: Shader,
        attribName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SetShaderValue(
        shader: Shader,
        locIndex: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_void,
        uniformType: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn SetShaderValueV(
        shader: Shader,
        locIndex: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_void,
        uniformType: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn SetShaderValueMatrix(shader: Shader, locIndex: ::std::os::raw::c_int, mat: Matrix);
}
unsafe extern "C" {
    pub fn SetShaderValueTexture(
        shader: Shader,
        locIndex: ::std::os::raw::c_int,
        texture: Texture2D,
    );
}
unsafe extern "C" {
    pub fn UnloadShader(shader: Shader);
}
unsafe extern "C" {
    pub fn GetScreenToWorldRay(position: Vector2, camera: Camera) -> Ray;
}
unsafe extern "C" {
    pub fn GetScreenToWorldRayEx(
        position: Vector2,
        camera: Camera,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> Ray;
}
unsafe extern "C" {
    pub fn GetWorldToScreen(position: Vector3, camera: Camera) -> Vector2;
}
unsafe extern "C" {
    pub fn GetWorldToScreenEx(
        position: Vector3,
        camera: Camera,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> Vector2;
}
unsafe extern "C" {
    pub fn GetWorldToScreen2D(position: Vector2, camera: Camera2D) -> Vector2;
}
unsafe extern "C" {
    pub fn GetScreenToWorld2D(position: Vector2, camera: Camera2D) -> Vector2;
}
unsafe extern "C" {
    pub fn GetCameraMatrix(camera: Camera) -> Matrix;
}
unsafe extern "C" {
    pub fn GetCameraMatrix2D(camera: Camera2D) -> Matrix;
}
unsafe extern "C" {
    pub fn SetTargetFPS(fps: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn GetFrameTime() -> f32;
}
unsafe extern "C" {
    pub fn GetTime() -> f64;
}
unsafe extern "C" {
    pub fn GetFPS() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SwapScreenBuffer();
}
unsafe extern "C" {
    pub fn PollInputEvents();
}
unsafe extern "C" {
    pub fn WaitTime(seconds: f64);
}
unsafe extern "C" {
    pub fn SetRandomSeed(seed: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn GetRandomValue(
        min: ::std::os::raw::c_int,
        max: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn LoadRandomSequence(
        count: ::std::os::raw::c_uint,
        min: ::std::os::raw::c_int,
        max: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn UnloadRandomSequence(sequence: *mut ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn TakeScreenshot(fileName: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn SetConfigFlags(flags: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn OpenURL(url: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn TraceLog(logLevel: ::std::os::raw::c_int, text: *const ::std::os::raw::c_char, ...);
}
unsafe extern "C" {
    pub fn SetTraceLogLevel(logLevel: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn MemAlloc(size: ::std::os::raw::c_uint) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn MemRealloc(
        ptr: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn MemFree(ptr: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn SetTraceLogCallback(callback: TraceLogCallback);
}
unsafe extern "C" {
    pub fn SetLoadFileDataCallback(callback: LoadFileDataCallback);
}
unsafe extern "C" {
    pub fn SetSaveFileDataCallback(callback: SaveFileDataCallback);
}
unsafe extern "C" {
    pub fn SetLoadFileTextCallback(callback: LoadFileTextCallback);
}
unsafe extern "C" {
    pub fn SetSaveFileTextCallback(callback: SaveFileTextCallback);
}
unsafe extern "C" {
    pub fn LoadFileData(
        fileName: *const ::std::os::raw::c_char,
        dataSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
unsafe extern "C" {
    pub fn UnloadFileData(data: *mut ::std::os::raw::c_uchar);
}
unsafe extern "C" {
    pub fn SaveFileData(
        fileName: *const ::std::os::raw::c_char,
        data: *mut ::std::os::raw::c_void,
        dataSize: ::std::os::raw::c_int,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ExportDataAsCode(
        data: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
        fileName: *const ::std::os::raw::c_char,
    ) -> bool;
}
unsafe extern "C" {
    pub fn LoadFileText(fileName: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn UnloadFileText(text: *mut ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn SaveFileText(
        fileName: *const ::std::os::raw::c_char,
        text: *mut ::std::os::raw::c_char,
    ) -> bool;
}
unsafe extern "C" {
    pub fn FileExists(fileName: *const ::std::os::raw::c_char) -> bool;
}
unsafe extern "C" {
    pub fn DirectoryExists(dirPath: *const ::std::os::raw::c_char) -> bool;
}
unsafe extern "C" {
    pub fn IsFileExtension(
        fileName: *const ::std::os::raw::c_char,
        ext: *const ::std::os::raw::c_char,
    ) -> bool;
}
unsafe extern "C" {
    pub fn GetFileLength(fileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetFileExtension(
        fileName: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn GetFileName(filePath: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn GetFileNameWithoutExt(
        filePath: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn GetDirectoryPath(
        filePath: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn GetPrevDirectoryPath(
        dirPath: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn GetWorkingDirectory() -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn GetApplicationDirectory() -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn MakeDirectory(dirPath: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn ChangeDirectory(dir: *const ::std::os::raw::c_char) -> bool;
}
unsafe extern "C" {
    pub fn IsPathFile(path: *const ::std::os::raw::c_char) -> bool;
}
unsafe extern "C" {
    pub fn IsFileNameValid(fileName: *const ::std::os::raw::c_char) -> bool;
}
unsafe extern "C" {
    pub fn LoadDirectoryFiles(dirPath: *const ::std::os::raw::c_char) -> FilePathList;
}
unsafe extern "C" {
    pub fn LoadDirectoryFilesEx(
        basePath: *const ::std::os::raw::c_char,
        filter: *const ::std::os::raw::c_char,
        scanSubdirs: bool,
    ) -> FilePathList;
}
unsafe extern "C" {
    pub fn UnloadDirectoryFiles(files: FilePathList);
}
unsafe extern "C" {
    pub fn IsFileDropped() -> bool;
}
unsafe extern "C" {
    pub fn LoadDroppedFiles() -> FilePathList;
}
unsafe extern "C" {
    pub fn UnloadDroppedFiles(files: FilePathList);
}
unsafe extern "C" {
    pub fn GetFileModTime(fileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn CompressData(
        data: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
        compDataSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
unsafe extern "C" {
    pub fn DecompressData(
        compData: *const ::std::os::raw::c_uchar,
        compDataSize: ::std::os::raw::c_int,
        dataSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
unsafe extern "C" {
    pub fn EncodeDataBase64(
        data: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
        outputSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn DecodeDataBase64(
        data: *const ::std::os::raw::c_uchar,
        outputSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
unsafe extern "C" {
    pub fn ComputeCRC32(
        data: *mut ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn ComputeMD5(
        data: *mut ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn ComputeSHA1(
        data: *mut ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn LoadAutomationEventList(fileName: *const ::std::os::raw::c_char) -> AutomationEventList;
}
unsafe extern "C" {
    pub fn UnloadAutomationEventList(list: AutomationEventList);
}
unsafe extern "C" {
    pub fn ExportAutomationEventList(
        list: AutomationEventList,
        fileName: *const ::std::os::raw::c_char,
    ) -> bool;
}
unsafe extern "C" {
    pub fn SetAutomationEventList(list: *mut AutomationEventList);
}
unsafe extern "C" {
    pub fn SetAutomationEventBaseFrame(frame: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn StartAutomationEventRecording();
}
unsafe extern "C" {
    pub fn StopAutomationEventRecording();
}
unsafe extern "C" {
    pub fn PlayAutomationEvent(event: AutomationEvent);
}
unsafe extern "C" {
    pub fn IsKeyPressed(key: ::std::os::raw::c_int) -> bool;
}
unsafe extern "C" {
    pub fn IsKeyPressedRepeat(key: ::std::os::raw::c_int) -> bool;
}
unsafe extern "C" {
    pub fn IsKeyDown(key: ::std::os::raw::c_int) -> bool;
}
unsafe extern "C" {
    pub fn IsKeyReleased(key: ::std::os::raw::c_int) -> bool;
}
unsafe extern "C" {
    pub fn IsKeyUp(key: ::std::os::raw::c_int) -> bool;
}
unsafe extern "C" {
    pub fn GetKeyPressed() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetCharPressed() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SetExitKey(key: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn IsGamepadAvailable(gamepad: ::std::os::raw::c_int) -> bool;
}
unsafe extern "C" {
    pub fn GetGamepadName(gamepad: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn IsGamepadButtonPressed(
        gamepad: ::std::os::raw::c_int,
        button: ::std::os::raw::c_int,
    ) -> bool;
}
unsafe extern "C" {
    pub fn IsGamepadButtonDown(
        gamepad: ::std::os::raw::c_int,
        button: ::std::os::raw::c_int,
    ) -> bool;
}
unsafe extern "C" {
    pub fn IsGamepadButtonReleased(
        gamepad: ::std::os::raw::c_int,
        button: ::std::os::raw::c_int,
    ) -> bool;
}
unsafe extern "C" {
    pub fn IsGamepadButtonUp(gamepad: ::std::os::raw::c_int, button: ::std::os::raw::c_int)
        -> bool;
}
unsafe extern "C" {
    pub fn GetGamepadButtonPressed() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetGamepadAxisCount(gamepad: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetGamepadAxisMovement(
        gamepad: ::std::os::raw::c_int,
        axis: ::std::os::raw::c_int,
    ) -> f32;
}
unsafe extern "C" {
    pub fn SetGamepadMappings(mappings: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SetGamepadVibration(
        gamepad: ::std::os::raw::c_int,
        leftMotor: f32,
        rightMotor: f32,
        duration: f32,
    );
}
unsafe extern "C" {
    pub fn IsMouseButtonPressed(button: ::std::os::raw::c_int) -> bool;
}
unsafe extern "C" {
    pub fn IsMouseButtonDown(button: ::std::os::raw::c_int) -> bool;
}
unsafe extern "C" {
    pub fn IsMouseButtonReleased(button: ::std::os::raw::c_int) -> bool;
}
unsafe extern "C" {
    pub fn IsMouseButtonUp(button: ::std::os::raw::c_int) -> bool;
}
unsafe extern "C" {
    pub fn GetMouseX() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetMouseY() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetMousePosition() -> Vector2;
}
unsafe extern "C" {
    pub fn GetMouseDelta() -> Vector2;
}
unsafe extern "C" {
    pub fn SetMousePosition(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn SetMouseOffset(offsetX: ::std::os::raw::c_int, offsetY: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn SetMouseScale(scaleX: f32, scaleY: f32);
}
unsafe extern "C" {
    pub fn GetMouseWheelMove() -> f32;
}
unsafe extern "C" {
    pub fn GetMouseWheelMoveV() -> Vector2;
}
unsafe extern "C" {
    pub fn SetMouseCursor(cursor: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn GetTouchX() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetTouchY() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetTouchPosition(index: ::std::os::raw::c_int) -> Vector2;
}
unsafe extern "C" {
    pub fn GetTouchPointId(index: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetTouchPointCount() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SetGesturesEnabled(flags: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn IsGestureDetected(gesture: ::std::os::raw::c_uint) -> bool;
}
unsafe extern "C" {
    pub fn GetGestureDetected() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetGestureHoldDuration() -> f32;
}
unsafe extern "C" {
    pub fn GetGestureDragVector() -> Vector2;
}
unsafe extern "C" {
    pub fn GetGestureDragAngle() -> f32;
}
unsafe extern "C" {
    pub fn GetGesturePinchVector() -> Vector2;
}
unsafe extern "C" {
    pub fn GetGesturePinchAngle() -> f32;
}
unsafe extern "C" {
    pub fn UpdateCamera(camera: *mut Camera, mode: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn UpdateCameraPro(camera: *mut Camera, movement: Vector3, rotation: Vector3, zoom: f32);
}
unsafe extern "C" {
    pub fn SetShapesTexture(texture: Texture2D, source: Rectangle);
}
unsafe extern "C" {
    pub fn GetShapesTexture() -> Texture2D;
}
unsafe extern "C" {
    pub fn GetShapesTextureRectangle() -> Rectangle;
}
unsafe extern "C" {
    pub fn DrawPixel(posX: ::std::os::raw::c_int, posY: ::std::os::raw::c_int, color: Color);
}
unsafe extern "C" {
    pub fn DrawPixelV(position: Vector2, color: Color);
}
unsafe extern "C" {
    pub fn DrawLine(
        startPosX: ::std::os::raw::c_int,
        startPosY: ::std::os::raw::c_int,
        endPosX: ::std::os::raw::c_int,
        endPosY: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawLineV(startPos: Vector2, endPos: Vector2, color: Color);
}
unsafe extern "C" {
    pub fn DrawLineEx(startPos: Vector2, endPos: Vector2, thick: f32, color: Color);
}
unsafe extern "C" {
    pub fn DrawLineStrip(points: *const Vector2, pointCount: ::std::os::raw::c_int, color: Color);
}
unsafe extern "C" {
    pub fn DrawLineBezier(startPos: Vector2, endPos: Vector2, thick: f32, color: Color);
}
unsafe extern "C" {
    pub fn DrawCircle(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: f32,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawCircleSector(
        center: Vector2,
        radius: f32,
        startAngle: f32,
        endAngle: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawCircleSectorLines(
        center: Vector2,
        radius: f32,
        startAngle: f32,
        endAngle: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawCircleGradient(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: f32,
        inner: Color,
        outer: Color,
    );
}
unsafe extern "C" {
    pub fn DrawCircleV(center: Vector2, radius: f32, color: Color);
}
unsafe extern "C" {
    pub fn DrawCircleLines(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: f32,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawCircleLinesV(center: Vector2, radius: f32, color: Color);
}
unsafe extern "C" {
    pub fn DrawEllipse(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radiusH: f32,
        radiusV: f32,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawEllipseLines(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radiusH: f32,
        radiusV: f32,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawRing(
        center: Vector2,
        innerRadius: f32,
        outerRadius: f32,
        startAngle: f32,
        endAngle: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawRingLines(
        center: Vector2,
        innerRadius: f32,
        outerRadius: f32,
        startAngle: f32,
        endAngle: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawRectangle(
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawRectangleV(position: Vector2, size: Vector2, color: Color);
}
unsafe extern "C" {
    pub fn DrawRectangleRec(rec: Rectangle, color: Color);
}
unsafe extern "C" {
    pub fn DrawRectanglePro(rec: Rectangle, origin: Vector2, rotation: f32, color: Color);
}
unsafe extern "C" {
    pub fn DrawRectangleGradientV(
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        top: Color,
        bottom: Color,
    );
}
unsafe extern "C" {
    pub fn DrawRectangleGradientH(
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        left: Color,
        right: Color,
    );
}
unsafe extern "C" {
    pub fn DrawRectangleGradientEx(
        rec: Rectangle,
        topLeft: Color,
        bottomLeft: Color,
        topRight: Color,
        bottomRight: Color,
    );
}
unsafe extern "C" {
    pub fn DrawRectangleLines(
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawRectangleLinesEx(rec: Rectangle, lineThick: f32, color: Color);
}
unsafe extern "C" {
    pub fn DrawRectangleRounded(
        rec: Rectangle,
        roundness: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawRectangleRoundedLines(
        rec: Rectangle,
        roundness: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawRectangleRoundedLinesEx(
        rec: Rectangle,
        roundness: f32,
        segments: ::std::os::raw::c_int,
        lineThick: f32,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawTriangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
}
unsafe extern "C" {
    pub fn DrawTriangleLines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
}
unsafe extern "C" {
    pub fn DrawTriangleFan(points: *const Vector2, pointCount: ::std::os::raw::c_int, color: Color);
}
unsafe extern "C" {
    pub fn DrawTriangleStrip(
        points: *const Vector2,
        pointCount: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawPoly(
        center: Vector2,
        sides: ::std::os::raw::c_int,
        radius: f32,
        rotation: f32,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawPolyLines(
        center: Vector2,
        sides: ::std::os::raw::c_int,
        radius: f32,
        rotation: f32,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawPolyLinesEx(
        center: Vector2,
        sides: ::std::os::raw::c_int,
        radius: f32,
        rotation: f32,
        lineThick: f32,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawSplineLinear(
        points: *const Vector2,
        pointCount: ::std::os::raw::c_int,
        thick: f32,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawSplineBasis(
        points: *const Vector2,
        pointCount: ::std::os::raw::c_int,
        thick: f32,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawSplineCatmullRom(
        points: *const Vector2,
        pointCount: ::std::os::raw::c_int,
        thick: f32,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawSplineBezierQuadratic(
        points: *const Vector2,
        pointCount: ::std::os::raw::c_int,
        thick: f32,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawSplineBezierCubic(
        points: *const Vector2,
        pointCount: ::std::os::raw::c_int,
        thick: f32,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawSplineSegmentLinear(p1: Vector2, p2: Vector2, thick: f32, color: Color);
}
unsafe extern "C" {
    pub fn DrawSplineSegmentBasis(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        thick: f32,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawSplineSegmentCatmullRom(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        thick: f32,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawSplineSegmentBezierQuadratic(
        p1: Vector2,
        c2: Vector2,
        p3: Vector2,
        thick: f32,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawSplineSegmentBezierCubic(
        p1: Vector2,
        c2: Vector2,
        c3: Vector2,
        p4: Vector2,
        thick: f32,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn GetSplinePointLinear(startPos: Vector2, endPos: Vector2, t: f32) -> Vector2;
}
unsafe extern "C" {
    pub fn GetSplinePointBasis(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        t: f32,
    ) -> Vector2;
}
unsafe extern "C" {
    pub fn GetSplinePointCatmullRom(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        t: f32,
    ) -> Vector2;
}
unsafe extern "C" {
    pub fn GetSplinePointBezierQuad(p1: Vector2, c2: Vector2, p3: Vector2, t: f32) -> Vector2;
}
unsafe extern "C" {
    pub fn GetSplinePointBezierCubic(
        p1: Vector2,
        c2: Vector2,
        c3: Vector2,
        p4: Vector2,
        t: f32,
    ) -> Vector2;
}
unsafe extern "C" {
    pub fn CheckCollisionRecs(rec1: Rectangle, rec2: Rectangle) -> bool;
}
unsafe extern "C" {
    pub fn CheckCollisionCircles(
        center1: Vector2,
        radius1: f32,
        center2: Vector2,
        radius2: f32,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CheckCollisionCircleRec(center: Vector2, radius: f32, rec: Rectangle) -> bool;
}
unsafe extern "C" {
    pub fn CheckCollisionCircleLine(center: Vector2, radius: f32, p1: Vector2, p2: Vector2)
        -> bool;
}
unsafe extern "C" {
    pub fn CheckCollisionPointRec(point: Vector2, rec: Rectangle) -> bool;
}
unsafe extern "C" {
    pub fn CheckCollisionPointCircle(point: Vector2, center: Vector2, radius: f32) -> bool;
}
unsafe extern "C" {
    pub fn CheckCollisionPointTriangle(
        point: Vector2,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CheckCollisionPointLine(
        point: Vector2,
        p1: Vector2,
        p2: Vector2,
        threshold: ::std::os::raw::c_int,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CheckCollisionPointPoly(
        point: Vector2,
        points: *const Vector2,
        pointCount: ::std::os::raw::c_int,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CheckCollisionLines(
        startPos1: Vector2,
        endPos1: Vector2,
        startPos2: Vector2,
        endPos2: Vector2,
        collisionPoint: *mut Vector2,
    ) -> bool;
}
unsafe extern "C" {
    pub fn GetCollisionRec(rec1: Rectangle, rec2: Rectangle) -> Rectangle;
}
unsafe extern "C" {
    pub fn LoadImage(fileName: *const ::std::os::raw::c_char) -> Image;
}
unsafe extern "C" {
    pub fn LoadImageRaw(
        fileName: *const ::std::os::raw::c_char,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
        headerSize: ::std::os::raw::c_int,
    ) -> Image;
}
unsafe extern "C" {
    pub fn LoadImageAnim(
        fileName: *const ::std::os::raw::c_char,
        frames: *mut ::std::os::raw::c_int,
    ) -> Image;
}
unsafe extern "C" {
    pub fn LoadImageAnimFromMemory(
        fileType: *const ::std::os::raw::c_char,
        fileData: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
        frames: *mut ::std::os::raw::c_int,
    ) -> Image;
}
unsafe extern "C" {
    pub fn LoadImageFromMemory(
        fileType: *const ::std::os::raw::c_char,
        fileData: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
    ) -> Image;
}
unsafe extern "C" {
    pub fn LoadImageFromTexture(texture: Texture2D) -> Image;
}
unsafe extern "C" {
    pub fn LoadImageFromScreen() -> Image;
}
unsafe extern "C" {
    pub fn IsImageValid(image: Image) -> bool;
}
unsafe extern "C" {
    pub fn UnloadImage(image: Image);
}
unsafe extern "C" {
    pub fn ExportImage(image: Image, fileName: *const ::std::os::raw::c_char) -> bool;
}
unsafe extern "C" {
    pub fn ExportImageToMemory(
        image: Image,
        fileType: *const ::std::os::raw::c_char,
        fileSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
unsafe extern "C" {
    pub fn ExportImageAsCode(image: Image, fileName: *const ::std::os::raw::c_char) -> bool;
}
unsafe extern "C" {
    pub fn GenImageColor(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: Color,
    ) -> Image;
}
unsafe extern "C" {
    pub fn GenImageGradientLinear(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        direction: ::std::os::raw::c_int,
        start: Color,
        end: Color,
    ) -> Image;
}
unsafe extern "C" {
    pub fn GenImageGradientRadial(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Image;
}
unsafe extern "C" {
    pub fn GenImageGradientSquare(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Image;
}
unsafe extern "C" {
    pub fn GenImageChecked(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        checksX: ::std::os::raw::c_int,
        checksY: ::std::os::raw::c_int,
        col1: Color,
        col2: Color,
    ) -> Image;
}
unsafe extern "C" {
    pub fn GenImageWhiteNoise(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        factor: f32,
    ) -> Image;
}
unsafe extern "C" {
    pub fn GenImagePerlinNoise(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        offsetX: ::std::os::raw::c_int,
        offsetY: ::std::os::raw::c_int,
        scale: f32,
    ) -> Image;
}
unsafe extern "C" {
    pub fn GenImageCellular(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        tileSize: ::std::os::raw::c_int,
    ) -> Image;
}
unsafe extern "C" {
    pub fn GenImageText(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        text: *const ::std::os::raw::c_char,
    ) -> Image;
}
unsafe extern "C" {
    pub fn ImageCopy(image: Image) -> Image;
}
unsafe extern "C" {
    pub fn ImageFromImage(image: Image, rec: Rectangle) -> Image;
}
unsafe extern "C" {
    pub fn ImageFromChannel(image: Image, selectedChannel: ::std::os::raw::c_int) -> Image;
}
unsafe extern "C" {
    pub fn ImageText(
        text: *const ::std::os::raw::c_char,
        fontSize: ::std::os::raw::c_int,
        color: Color,
    ) -> Image;
}
unsafe extern "C" {
    pub fn ImageTextEx(
        font: Font,
        text: *const ::std::os::raw::c_char,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    ) -> Image;
}
unsafe extern "C" {
    pub fn ImageFormat(image: *mut Image, newFormat: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn ImageToPOT(image: *mut Image, fill: Color);
}
unsafe extern "C" {
    pub fn ImageCrop(image: *mut Image, crop: Rectangle);
}
unsafe extern "C" {
    pub fn ImageAlphaCrop(image: *mut Image, threshold: f32);
}
unsafe extern "C" {
    pub fn ImageAlphaClear(image: *mut Image, color: Color, threshold: f32);
}
unsafe extern "C" {
    pub fn ImageAlphaMask(image: *mut Image, alphaMask: Image);
}
unsafe extern "C" {
    pub fn ImageAlphaPremultiply(image: *mut Image);
}
unsafe extern "C" {
    pub fn ImageBlurGaussian(image: *mut Image, blurSize: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn ImageKernelConvolution(
        image: *mut Image,
        kernel: *const f32,
        kernelSize: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn ImageResize(
        image: *mut Image,
        newWidth: ::std::os::raw::c_int,
        newHeight: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn ImageResizeNN(
        image: *mut Image,
        newWidth: ::std::os::raw::c_int,
        newHeight: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn ImageResizeCanvas(
        image: *mut Image,
        newWidth: ::std::os::raw::c_int,
        newHeight: ::std::os::raw::c_int,
        offsetX: ::std::os::raw::c_int,
        offsetY: ::std::os::raw::c_int,
        fill: Color,
    );
}
unsafe extern "C" {
    pub fn ImageMipmaps(image: *mut Image);
}
unsafe extern "C" {
    pub fn ImageDither(
        image: *mut Image,
        rBpp: ::std::os::raw::c_int,
        gBpp: ::std::os::raw::c_int,
        bBpp: ::std::os::raw::c_int,
        aBpp: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn ImageFlipVertical(image: *mut Image);
}
unsafe extern "C" {
    pub fn ImageFlipHorizontal(image: *mut Image);
}
unsafe extern "C" {
    pub fn ImageRotate(image: *mut Image, degrees: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn ImageRotateCW(image: *mut Image);
}
unsafe extern "C" {
    pub fn ImageRotateCCW(image: *mut Image);
}
unsafe extern "C" {
    pub fn ImageColorTint(image: *mut Image, color: Color);
}
unsafe extern "C" {
    pub fn ImageColorInvert(image: *mut Image);
}
unsafe extern "C" {
    pub fn ImageColorGrayscale(image: *mut Image);
}
unsafe extern "C" {
    pub fn ImageColorContrast(image: *mut Image, contrast: f32);
}
unsafe extern "C" {
    pub fn ImageColorBrightness(image: *mut Image, brightness: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn ImageColorReplace(image: *mut Image, color: Color, replace: Color);
}
unsafe extern "C" {
    pub fn LoadImageColors(image: Image) -> *mut Color;
}
unsafe extern "C" {
    pub fn LoadImagePalette(
        image: Image,
        maxPaletteSize: ::std::os::raw::c_int,
        colorCount: *mut ::std::os::raw::c_int,
    ) -> *mut Color;
}
unsafe extern "C" {
    pub fn UnloadImageColors(colors: *mut Color);
}
unsafe extern "C" {
    pub fn UnloadImagePalette(colors: *mut Color);
}
unsafe extern "C" {
    pub fn GetImageAlphaBorder(image: Image, threshold: f32) -> Rectangle;
}
unsafe extern "C" {
    pub fn GetImageColor(image: Image, x: ::std::os::raw::c_int, y: ::std::os::raw::c_int)
        -> Color;
}
unsafe extern "C" {
    pub fn ImageClearBackground(dst: *mut Image, color: Color);
}
unsafe extern "C" {
    pub fn ImageDrawPixel(
        dst: *mut Image,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn ImageDrawPixelV(dst: *mut Image, position: Vector2, color: Color);
}
unsafe extern "C" {
    pub fn ImageDrawLine(
        dst: *mut Image,
        startPosX: ::std::os::raw::c_int,
        startPosY: ::std::os::raw::c_int,
        endPosX: ::std::os::raw::c_int,
        endPosY: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn ImageDrawLineV(dst: *mut Image, start: Vector2, end: Vector2, color: Color);
}
unsafe extern "C" {
    pub fn ImageDrawLineEx(
        dst: *mut Image,
        start: Vector2,
        end: Vector2,
        thick: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn ImageDrawCircle(
        dst: *mut Image,
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn ImageDrawCircleV(
        dst: *mut Image,
        center: Vector2,
        radius: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn ImageDrawCircleLines(
        dst: *mut Image,
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn ImageDrawCircleLinesV(
        dst: *mut Image,
        center: Vector2,
        radius: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn ImageDrawRectangle(
        dst: *mut Image,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn ImageDrawRectangleV(dst: *mut Image, position: Vector2, size: Vector2, color: Color);
}
unsafe extern "C" {
    pub fn ImageDrawRectangleRec(dst: *mut Image, rec: Rectangle, color: Color);
}
unsafe extern "C" {
    pub fn ImageDrawRectangleLines(
        dst: *mut Image,
        rec: Rectangle,
        thick: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn ImageDrawTriangle(dst: *mut Image, v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
}
unsafe extern "C" {
    pub fn ImageDrawTriangleEx(
        dst: *mut Image,
        v1: Vector2,
        v2: Vector2,
        v3: Vector2,
        c1: Color,
        c2: Color,
        c3: Color,
    );
}
unsafe extern "C" {
    pub fn ImageDrawTriangleLines(
        dst: *mut Image,
        v1: Vector2,
        v2: Vector2,
        v3: Vector2,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn ImageDrawTriangleFan(
        dst: *mut Image,
        points: *mut Vector2,
        pointCount: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn ImageDrawTriangleStrip(
        dst: *mut Image,
        points: *mut Vector2,
        pointCount: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn ImageDraw(
        dst: *mut Image,
        src: Image,
        srcRec: Rectangle,
        dstRec: Rectangle,
        tint: Color,
    );
}
unsafe extern "C" {
    pub fn ImageDrawText(
        dst: *mut Image,
        text: *const ::std::os::raw::c_char,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn ImageDrawTextEx(
        dst: *mut Image,
        font: Font,
        text: *const ::std::os::raw::c_char,
        position: Vector2,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    );
}
unsafe extern "C" {
    pub fn LoadTexture(fileName: *const ::std::os::raw::c_char) -> Texture2D;
}
unsafe extern "C" {
    pub fn LoadTextureFromImage(image: Image) -> Texture2D;
}
unsafe extern "C" {
    pub fn LoadTextureCubemap(image: Image, layout: ::std::os::raw::c_int) -> TextureCubemap;
}
unsafe extern "C" {
    pub fn LoadRenderTexture(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> RenderTexture2D;
}
unsafe extern "C" {
    pub fn IsTextureValid(texture: Texture2D) -> bool;
}
unsafe extern "C" {
    pub fn UnloadTexture(texture: Texture2D);
}
unsafe extern "C" {
    pub fn IsRenderTextureValid(target: RenderTexture2D) -> bool;
}
unsafe extern "C" {
    pub fn UnloadRenderTexture(target: RenderTexture2D);
}
unsafe extern "C" {
    pub fn UpdateTexture(texture: Texture2D, pixels: *const ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn UpdateTextureRec(
        texture: Texture2D,
        rec: Rectangle,
        pixels: *const ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn GenTextureMipmaps(texture: *mut Texture2D);
}
unsafe extern "C" {
    pub fn SetTextureFilter(texture: Texture2D, filter: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn SetTextureWrap(texture: Texture2D, wrap: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn DrawTexture(
        texture: Texture2D,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        tint: Color,
    );
}
unsafe extern "C" {
    pub fn DrawTextureV(texture: Texture2D, position: Vector2, tint: Color);
}
unsafe extern "C" {
    pub fn DrawTextureEx(
        texture: Texture2D,
        position: Vector2,
        rotation: f32,
        scale: f32,
        tint: Color,
    );
}
unsafe extern "C" {
    pub fn DrawTextureRec(texture: Texture2D, source: Rectangle, position: Vector2, tint: Color);
}
unsafe extern "C" {
    pub fn DrawTexturePro(
        texture: Texture2D,
        source: Rectangle,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    );
}
unsafe extern "C" {
    pub fn DrawTextureNPatch(
        texture: Texture2D,
        nPatchInfo: NPatchInfo,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    );
}
unsafe extern "C" {
    pub fn ColorIsEqual(col1: Color, col2: Color) -> bool;
}
unsafe extern "C" {
    pub fn Fade(color: Color, alpha: f32) -> Color;
}
unsafe extern "C" {
    pub fn ColorToInt(color: Color) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn ColorNormalize(color: Color) -> Vector4;
}
unsafe extern "C" {
    pub fn ColorFromNormalized(normalized: Vector4) -> Color;
}
unsafe extern "C" {
    pub fn ColorToHSV(color: Color) -> Vector3;
}
unsafe extern "C" {
    pub fn ColorFromHSV(hue: f32, saturation: f32, value: f32) -> Color;
}
unsafe extern "C" {
    pub fn ColorTint(color: Color, tint: Color) -> Color;
}
unsafe extern "C" {
    pub fn ColorBrightness(color: Color, factor: f32) -> Color;
}
unsafe extern "C" {
    pub fn ColorContrast(color: Color, contrast: f32) -> Color;
}
unsafe extern "C" {
    pub fn ColorAlpha(color: Color, alpha: f32) -> Color;
}
unsafe extern "C" {
    pub fn ColorAlphaBlend(dst: Color, src: Color, tint: Color) -> Color;
}
unsafe extern "C" {
    pub fn ColorLerp(color1: Color, color2: Color, factor: f32) -> Color;
}
unsafe extern "C" {
    pub fn GetColor(hexValue: ::std::os::raw::c_uint) -> Color;
}
unsafe extern "C" {
    pub fn GetPixelColor(
        srcPtr: *mut ::std::os::raw::c_void,
        format: ::std::os::raw::c_int,
    ) -> Color;
}
unsafe extern "C" {
    pub fn SetPixelColor(
        dstPtr: *mut ::std::os::raw::c_void,
        color: Color,
        format: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn GetPixelDataSize(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetFontDefault() -> Font;
}
unsafe extern "C" {
    pub fn LoadFont(fileName: *const ::std::os::raw::c_char) -> Font;
}
unsafe extern "C" {
    pub fn LoadFontEx(
        fileName: *const ::std::os::raw::c_char,
        fontSize: ::std::os::raw::c_int,
        codepoints: *mut ::std::os::raw::c_int,
        codepointCount: ::std::os::raw::c_int,
    ) -> Font;
}
unsafe extern "C" {
    pub fn LoadFontFromImage(image: Image, key: Color, firstChar: ::std::os::raw::c_int) -> Font;
}
unsafe extern "C" {
    pub fn LoadFontFromMemory(
        fileType: *const ::std::os::raw::c_char,
        fileData: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        codepoints: *mut ::std::os::raw::c_int,
        codepointCount: ::std::os::raw::c_int,
    ) -> Font;
}
unsafe extern "C" {
    pub fn IsFontValid(font: Font) -> bool;
}
unsafe extern "C" {
    pub fn LoadFontData(
        fileData: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        codepoints: *mut ::std::os::raw::c_int,
        codepointCount: ::std::os::raw::c_int,
        type_: ::std::os::raw::c_int,
    ) -> *mut GlyphInfo;
}
unsafe extern "C" {
    pub fn GenImageFontAtlas(
        glyphs: *const GlyphInfo,
        glyphRecs: *mut *mut Rectangle,
        glyphCount: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        padding: ::std::os::raw::c_int,
        packMethod: ::std::os::raw::c_int,
    ) -> Image;
}
unsafe extern "C" {
    pub fn UnloadFontData(glyphs: *mut GlyphInfo, glyphCount: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn UnloadFont(font: Font);
}
unsafe extern "C" {
    pub fn ExportFontAsCode(font: Font, fileName: *const ::std::os::raw::c_char) -> bool;
}
unsafe extern "C" {
    pub fn DrawFPS(posX: ::std::os::raw::c_int, posY: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn DrawText(
        text: *const ::std::os::raw::c_char,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawTextEx(
        font: Font,
        text: *const ::std::os::raw::c_char,
        position: Vector2,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    );
}
unsafe extern "C" {
    pub fn DrawTextPro(
        font: Font,
        text: *const ::std::os::raw::c_char,
        position: Vector2,
        origin: Vector2,
        rotation: f32,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    );
}
unsafe extern "C" {
    pub fn DrawTextCodepoint(
        font: Font,
        codepoint: ::std::os::raw::c_int,
        position: Vector2,
        fontSize: f32,
        tint: Color,
    );
}
unsafe extern "C" {
    pub fn DrawTextCodepoints(
        font: Font,
        codepoints: *const ::std::os::raw::c_int,
        codepointCount: ::std::os::raw::c_int,
        position: Vector2,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    );
}
unsafe extern "C" {
    pub fn SetTextLineSpacing(spacing: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn MeasureText(
        text: *const ::std::os::raw::c_char,
        fontSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn MeasureTextEx(
        font: Font,
        text: *const ::std::os::raw::c_char,
        fontSize: f32,
        spacing: f32,
    ) -> Vector2;
}
unsafe extern "C" {
    pub fn GetGlyphIndex(font: Font, codepoint: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetGlyphInfo(font: Font, codepoint: ::std::os::raw::c_int) -> GlyphInfo;
}
unsafe extern "C" {
    pub fn GetGlyphAtlasRec(font: Font, codepoint: ::std::os::raw::c_int) -> Rectangle;
}
unsafe extern "C" {
    pub fn LoadUTF8(
        codepoints: *const ::std::os::raw::c_int,
        length: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn UnloadUTF8(text: *mut ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn LoadCodepoints(
        text: *const ::std::os::raw::c_char,
        count: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn UnloadCodepoints(codepoints: *mut ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn GetCodepointCount(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetCodepoint(
        text: *const ::std::os::raw::c_char,
        codepointSize: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetCodepointNext(
        text: *const ::std::os::raw::c_char,
        codepointSize: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GetCodepointPrevious(
        text: *const ::std::os::raw::c_char,
        codepointSize: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn CodepointToUTF8(
        codepoint: ::std::os::raw::c_int,
        utf8Size: *mut ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn TextCopy(
        dst: *mut ::std::os::raw::c_char,
        src: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn TextIsEqual(
        text1: *const ::std::os::raw::c_char,
        text2: *const ::std::os::raw::c_char,
    ) -> bool;
}
unsafe extern "C" {
    pub fn TextLength(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn TextFormat(text: *const ::std::os::raw::c_char, ...) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn TextSubtext(
        text: *const ::std::os::raw::c_char,
        position: ::std::os::raw::c_int,
        length: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn TextReplace(
        text: *const ::std::os::raw::c_char,
        replace: *const ::std::os::raw::c_char,
        by: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn TextInsert(
        text: *const ::std::os::raw::c_char,
        insert: *const ::std::os::raw::c_char,
        position: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn TextJoin(
        textList: *mut *const ::std::os::raw::c_char,
        count: ::std::os::raw::c_int,
        delimiter: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn TextSplit(
        text: *const ::std::os::raw::c_char,
        delimiter: ::std::os::raw::c_char,
        count: *mut ::std::os::raw::c_int,
    ) -> *mut *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn TextAppend(
        text: *mut ::std::os::raw::c_char,
        append: *const ::std::os::raw::c_char,
        position: *mut ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn TextFindIndex(
        text: *const ::std::os::raw::c_char,
        find: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn TextToUpper(text: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn TextToLower(text: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn TextToPascal(text: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn TextToSnake(text: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn TextToCamel(text: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn TextToInteger(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn TextToFloat(text: *const ::std::os::raw::c_char) -> f32;
}
unsafe extern "C" {
    pub fn DrawLine3D(startPos: Vector3, endPos: Vector3, color: Color);
}
unsafe extern "C" {
    pub fn DrawPoint3D(position: Vector3, color: Color);
}
unsafe extern "C" {
    pub fn DrawCircle3D(
        center: Vector3,
        radius: f32,
        rotationAxis: Vector3,
        rotationAngle: f32,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawTriangle3D(v1: Vector3, v2: Vector3, v3: Vector3, color: Color);
}
unsafe extern "C" {
    pub fn DrawTriangleStrip3D(
        points: *const Vector3,
        pointCount: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawCube(position: Vector3, width: f32, height: f32, length: f32, color: Color);
}
unsafe extern "C" {
    pub fn DrawCubeV(position: Vector3, size: Vector3, color: Color);
}
unsafe extern "C" {
    pub fn DrawCubeWires(position: Vector3, width: f32, height: f32, length: f32, color: Color);
}
unsafe extern "C" {
    pub fn DrawCubeWiresV(position: Vector3, size: Vector3, color: Color);
}
unsafe extern "C" {
    pub fn DrawSphere(centerPos: Vector3, radius: f32, color: Color);
}
unsafe extern "C" {
    pub fn DrawSphereEx(
        centerPos: Vector3,
        radius: f32,
        rings: ::std::os::raw::c_int,
        slices: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawSphereWires(
        centerPos: Vector3,
        radius: f32,
        rings: ::std::os::raw::c_int,
        slices: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawCylinder(
        position: Vector3,
        radiusTop: f32,
        radiusBottom: f32,
        height: f32,
        slices: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawCylinderEx(
        startPos: Vector3,
        endPos: Vector3,
        startRadius: f32,
        endRadius: f32,
        sides: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawCylinderWires(
        position: Vector3,
        radiusTop: f32,
        radiusBottom: f32,
        height: f32,
        slices: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawCylinderWiresEx(
        startPos: Vector3,
        endPos: Vector3,
        startRadius: f32,
        endRadius: f32,
        sides: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawCapsule(
        startPos: Vector3,
        endPos: Vector3,
        radius: f32,
        slices: ::std::os::raw::c_int,
        rings: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawCapsuleWires(
        startPos: Vector3,
        endPos: Vector3,
        radius: f32,
        slices: ::std::os::raw::c_int,
        rings: ::std::os::raw::c_int,
        color: Color,
    );
}
unsafe extern "C" {
    pub fn DrawPlane(centerPos: Vector3, size: Vector2, color: Color);
}
unsafe extern "C" {
    pub fn DrawRay(ray: Ray, color: Color);
}
unsafe extern "C" {
    pub fn DrawGrid(slices: ::std::os::raw::c_int, spacing: f32);
}
unsafe extern "C" {
    pub fn LoadModel(fileName: *const ::std::os::raw::c_char) -> Model;
}
unsafe extern "C" {
    pub fn LoadModelFromMesh(mesh: Mesh) -> Model;
}
unsafe extern "C" {
    pub fn IsModelValid(model: Model) -> bool;
}
unsafe extern "C" {
    pub fn UnloadModel(model: Model);
}
unsafe extern "C" {
    pub fn GetModelBoundingBox(model: Model) -> BoundingBox;
}
unsafe extern "C" {
    pub fn DrawModel(model: Model, position: Vector3, scale: f32, tint: Color);
}
unsafe extern "C" {
    pub fn DrawModelEx(
        model: Model,
        position: Vector3,
        rotationAxis: Vector3,
        rotationAngle: f32,
        scale: Vector3,
        tint: Color,
    );
}
unsafe extern "C" {
    pub fn DrawModelWires(model: Model, position: Vector3, scale: f32, tint: Color);
}
unsafe extern "C" {
    pub fn DrawModelWiresEx(
        model: Model,
        position: Vector3,
        rotationAxis: Vector3,
        rotationAngle: f32,
        scale: Vector3,
        tint: Color,
    );
}
unsafe extern "C" {
    pub fn DrawModelPoints(model: Model, position: Vector3, scale: f32, tint: Color);
}
unsafe extern "C" {
    pub fn DrawModelPointsEx(
        model: Model,
        position: Vector3,
        rotationAxis: Vector3,
        rotationAngle: f32,
        scale: Vector3,
        tint: Color,
    );
}
unsafe extern "C" {
    pub fn DrawBoundingBox(box_: BoundingBox, color: Color);
}
unsafe extern "C" {
    pub fn DrawBillboard(
        camera: Camera,
        texture: Texture2D,
        position: Vector3,
        scale: f32,
        tint: Color,
    );
}
unsafe extern "C" {
    pub fn DrawBillboardRec(
        camera: Camera,
        texture: Texture2D,
        source: Rectangle,
        position: Vector3,
        size: Vector2,
        tint: Color,
    );
}
unsafe extern "C" {
    pub fn DrawBillboardPro(
        camera: Camera,
        texture: Texture2D,
        source: Rectangle,
        position: Vector3,
        up: Vector3,
        size: Vector2,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    );
}
unsafe extern "C" {
    pub fn UploadMesh(mesh: *mut Mesh, dynamic: bool);
}
unsafe extern "C" {
    pub fn UpdateMeshBuffer(
        mesh: Mesh,
        index: ::std::os::raw::c_int,
        data: *const ::std::os::raw::c_void,
        dataSize: ::std::os::raw::c_int,
        offset: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn UnloadMesh(mesh: Mesh);
}
unsafe extern "C" {
    pub fn DrawMesh(mesh: Mesh, material: Material, transform: Matrix);
}
unsafe extern "C" {
    pub fn DrawMeshInstanced(
        mesh: Mesh,
        material: Material,
        transforms: *const Matrix,
        instances: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn GetMeshBoundingBox(mesh: Mesh) -> BoundingBox;
}
unsafe extern "C" {
    pub fn GenMeshTangents(mesh: *mut Mesh);
}
unsafe extern "C" {
    pub fn ExportMesh(mesh: Mesh, fileName: *const ::std::os::raw::c_char) -> bool;
}
unsafe extern "C" {
    pub fn ExportMeshAsCode(mesh: Mesh, fileName: *const ::std::os::raw::c_char) -> bool;
}
unsafe extern "C" {
    pub fn GenMeshPoly(sides: ::std::os::raw::c_int, radius: f32) -> Mesh;
}
unsafe extern "C" {
    pub fn GenMeshPlane(
        width: f32,
        length: f32,
        resX: ::std::os::raw::c_int,
        resZ: ::std::os::raw::c_int,
    ) -> Mesh;
}
unsafe extern "C" {
    pub fn GenMeshCube(width: f32, height: f32, length: f32) -> Mesh;
}
unsafe extern "C" {
    pub fn GenMeshSphere(
        radius: f32,
        rings: ::std::os::raw::c_int,
        slices: ::std::os::raw::c_int,
    ) -> Mesh;
}
unsafe extern "C" {
    pub fn GenMeshHemiSphere(
        radius: f32,
        rings: ::std::os::raw::c_int,
        slices: ::std::os::raw::c_int,
    ) -> Mesh;
}
unsafe extern "C" {
    pub fn GenMeshCylinder(radius: f32, height: f32, slices: ::std::os::raw::c_int) -> Mesh;
}
unsafe extern "C" {
    pub fn GenMeshCone(radius: f32, height: f32, slices: ::std::os::raw::c_int) -> Mesh;
}
unsafe extern "C" {
    pub fn GenMeshTorus(
        radius: f32,
        size: f32,
        radSeg: ::std::os::raw::c_int,
        sides: ::std::os::raw::c_int,
    ) -> Mesh;
}
unsafe extern "C" {
    pub fn GenMeshKnot(
        radius: f32,
        size: f32,
        radSeg: ::std::os::raw::c_int,
        sides: ::std::os::raw::c_int,
    ) -> Mesh;
}
unsafe extern "C" {
    pub fn GenMeshHeightmap(heightmap: Image, size: Vector3) -> Mesh;
}
unsafe extern "C" {
    pub fn GenMeshCubicmap(cubicmap: Image, cubeSize: Vector3) -> Mesh;
}
unsafe extern "C" {
    pub fn LoadMaterials(
        fileName: *const ::std::os::raw::c_char,
        materialCount: *mut ::std::os::raw::c_int,
    ) -> *mut Material;
}
unsafe extern "C" {
    pub fn LoadMaterialDefault() -> Material;
}
unsafe extern "C" {
    pub fn IsMaterialValid(material: Material) -> bool;
}
unsafe extern "C" {
    pub fn UnloadMaterial(material: Material);
}
unsafe extern "C" {
    pub fn SetMaterialTexture(
        material: *mut Material,
        mapType: ::std::os::raw::c_int,
        texture: Texture2D,
    );
}
unsafe extern "C" {
    pub fn SetModelMeshMaterial(
        model: *mut Model,
        meshId: ::std::os::raw::c_int,
        materialId: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn LoadModelAnimations(
        fileName: *const ::std::os::raw::c_char,
        animCount: *mut ::std::os::raw::c_int,
    ) -> *mut ModelAnimation;
}
unsafe extern "C" {
    pub fn UpdateModelAnimation(model: Model, anim: ModelAnimation, frame: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn UpdateModelAnimationBones(
        model: Model,
        anim: ModelAnimation,
        frame: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn UnloadModelAnimation(anim: ModelAnimation);
}
unsafe extern "C" {
    pub fn UnloadModelAnimations(animations: *mut ModelAnimation, animCount: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn IsModelAnimationValid(model: Model, anim: ModelAnimation) -> bool;
}
unsafe extern "C" {
    pub fn CheckCollisionSpheres(
        center1: Vector3,
        radius1: f32,
        center2: Vector3,
        radius2: f32,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CheckCollisionBoxes(box1: BoundingBox, box2: BoundingBox) -> bool;
}
unsafe extern "C" {
    pub fn CheckCollisionBoxSphere(box_: BoundingBox, center: Vector3, radius: f32) -> bool;
}
unsafe extern "C" {
    pub fn GetRayCollisionSphere(ray: Ray, center: Vector3, radius: f32) -> RayCollision;
}
unsafe extern "C" {
    pub fn GetRayCollisionBox(ray: Ray, box_: BoundingBox) -> RayCollision;
}
unsafe extern "C" {
    pub fn GetRayCollisionMesh(ray: Ray, mesh: Mesh, transform: Matrix) -> RayCollision;
}
unsafe extern "C" {
    pub fn GetRayCollisionTriangle(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3)
        -> RayCollision;
}
unsafe extern "C" {
    pub fn GetRayCollisionQuad(
        ray: Ray,
        p1: Vector3,
        p2: Vector3,
        p3: Vector3,
        p4: Vector3,
    ) -> RayCollision;
}
pub type AudioCallback = ::std::option::Option<
    unsafe extern "C" fn(bufferData: *mut ::std::os::raw::c_void, frames: ::std::os::raw::c_uint),
>;
unsafe extern "C" {
    pub fn InitAudioDevice();
}
unsafe extern "C" {
    pub fn CloseAudioDevice();
}
unsafe extern "C" {
    pub fn IsAudioDeviceReady() -> bool;
}
unsafe extern "C" {
    pub fn SetMasterVolume(volume: f32);
}
unsafe extern "C" {
    pub fn GetMasterVolume() -> f32;
}
unsafe extern "C" {
    pub fn LoadWave(fileName: *const ::std::os::raw::c_char) -> Wave;
}
unsafe extern "C" {
    pub fn LoadWaveFromMemory(
        fileType: *const ::std::os::raw::c_char,
        fileData: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
    ) -> Wave;
}
unsafe extern "C" {
    pub fn IsWaveValid(wave: Wave) -> bool;
}
unsafe extern "C" {
    pub fn LoadSound(fileName: *const ::std::os::raw::c_char) -> Sound;
}
unsafe extern "C" {
    pub fn LoadSoundFromWave(wave: Wave) -> Sound;
}
unsafe extern "C" {
    pub fn LoadSoundAlias(source: Sound) -> Sound;
}
unsafe extern "C" {
    pub fn IsSoundValid(sound: Sound) -> bool;
}
unsafe extern "C" {
    pub fn UpdateSound(
        sound: Sound,
        data: *const ::std::os::raw::c_void,
        sampleCount: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn UnloadWave(wave: Wave);
}
unsafe extern "C" {
    pub fn UnloadSound(sound: Sound);
}
unsafe extern "C" {
    pub fn UnloadSoundAlias(alias: Sound);
}
unsafe extern "C" {
    pub fn ExportWave(wave: Wave, fileName: *const ::std::os::raw::c_char) -> bool;
}
unsafe extern "C" {
    pub fn ExportWaveAsCode(wave: Wave, fileName: *const ::std::os::raw::c_char) -> bool;
}
unsafe extern "C" {
    pub fn PlaySound(sound: Sound);
}
unsafe extern "C" {
    pub fn StopSound(sound: Sound);
}
unsafe extern "C" {
    pub fn PauseSound(sound: Sound);
}
unsafe extern "C" {
    pub fn ResumeSound(sound: Sound);
}
unsafe extern "C" {
    pub fn IsSoundPlaying(sound: Sound) -> bool;
}
unsafe extern "C" {
    pub fn SetSoundVolume(sound: Sound, volume: f32);
}
unsafe extern "C" {
    pub fn SetSoundPitch(sound: Sound, pitch: f32);
}
unsafe extern "C" {
    pub fn SetSoundPan(sound: Sound, pan: f32);
}
unsafe extern "C" {
    pub fn WaveCopy(wave: Wave) -> Wave;
}
unsafe extern "C" {
    pub fn WaveCrop(
        wave: *mut Wave,
        initFrame: ::std::os::raw::c_int,
        finalFrame: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn WaveFormat(
        wave: *mut Wave,
        sampleRate: ::std::os::raw::c_int,
        sampleSize: ::std::os::raw::c_int,
        channels: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn LoadWaveSamples(wave: Wave) -> *mut f32;
}
unsafe extern "C" {
    pub fn UnloadWaveSamples(samples: *mut f32);
}
unsafe extern "C" {
    pub fn LoadMusicStream(fileName: *const ::std::os::raw::c_char) -> Music;
}
unsafe extern "C" {
    pub fn LoadMusicStreamFromMemory(
        fileType: *const ::std::os::raw::c_char,
        data: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
    ) -> Music;
}
unsafe extern "C" {
    pub fn IsMusicValid(music: Music) -> bool;
}
unsafe extern "C" {
    pub fn UnloadMusicStream(music: Music);
}
unsafe extern "C" {
    pub fn PlayMusicStream(music: Music);
}
unsafe extern "C" {
    pub fn IsMusicStreamPlaying(music: Music) -> bool;
}
unsafe extern "C" {
    pub fn UpdateMusicStream(music: Music);
}
unsafe extern "C" {
    pub fn StopMusicStream(music: Music);
}
unsafe extern "C" {
    pub fn PauseMusicStream(music: Music);
}
unsafe extern "C" {
    pub fn ResumeMusicStream(music: Music);
}
unsafe extern "C" {
    pub fn SeekMusicStream(music: Music, position: f32);
}
unsafe extern "C" {
    pub fn SetMusicVolume(music: Music, volume: f32);
}
unsafe extern "C" {
    pub fn SetMusicPitch(music: Music, pitch: f32);
}
unsafe extern "C" {
    pub fn SetMusicPan(music: Music, pan: f32);
}
unsafe extern "C" {
    pub fn GetMusicTimeLength(music: Music) -> f32;
}
unsafe extern "C" {
    pub fn GetMusicTimePlayed(music: Music) -> f32;
}
unsafe extern "C" {
    pub fn LoadAudioStream(
        sampleRate: ::std::os::raw::c_uint,
        sampleSize: ::std::os::raw::c_uint,
        channels: ::std::os::raw::c_uint,
    ) -> AudioStream;
}
unsafe extern "C" {
    pub fn IsAudioStreamValid(stream: AudioStream) -> bool;
}
unsafe extern "C" {
    pub fn UnloadAudioStream(stream: AudioStream);
}
unsafe extern "C" {
    pub fn UpdateAudioStream(
        stream: AudioStream,
        data: *const ::std::os::raw::c_void,
        frameCount: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn IsAudioStreamProcessed(stream: AudioStream) -> bool;
}
unsafe extern "C" {
    pub fn PlayAudioStream(stream: AudioStream);
}
unsafe extern "C" {
    pub fn PauseAudioStream(stream: AudioStream);
}
unsafe extern "C" {
    pub fn ResumeAudioStream(stream: AudioStream);
}
unsafe extern "C" {
    pub fn IsAudioStreamPlaying(stream: AudioStream) -> bool;
}
unsafe extern "C" {
    pub fn StopAudioStream(stream: AudioStream);
}
unsafe extern "C" {
    pub fn SetAudioStreamVolume(stream: AudioStream, volume: f32);
}
unsafe extern "C" {
    pub fn SetAudioStreamPitch(stream: AudioStream, pitch: f32);
}
unsafe extern "C" {
    pub fn SetAudioStreamPan(stream: AudioStream, pan: f32);
}
unsafe extern "C" {
    pub fn SetAudioStreamBufferSizeDefault(size: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn SetAudioStreamCallback(stream: AudioStream, callback: AudioCallback);
}
unsafe extern "C" {
    pub fn AttachAudioStreamProcessor(stream: AudioStream, processor: AudioCallback);
}
unsafe extern "C" {
    pub fn DetachAudioStreamProcessor(stream: AudioStream, processor: AudioCallback);
}
unsafe extern "C" {
    pub fn AttachAudioMixedProcessor(processor: AudioCallback);
}
unsafe extern "C" {
    pub fn DetachAudioMixedProcessor(processor: AudioCallback);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct float3 {
    pub v: [f32; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct float16 {
    pub v: [f32; 16usize],
}
pub type _Float128 = u128;
pub type _Float32 = f32;
pub type _Float64 = f64;
pub type _Float32x = f64;
pub type _Float64x = u128;
pub type float_t = f32;
pub type double_t = f64;
unsafe extern "C" {
    pub fn Clamp(value: f32, min: f32, max: f32) -> f32;
}
unsafe extern "C" {
    pub fn Lerp(start: f32, end: f32, amount: f32) -> f32;
}
unsafe extern "C" {
    pub fn Normalize(value: f32, start: f32, end: f32) -> f32;
}
unsafe extern "C" {
    pub fn Remap(
        value: f32,
        inputStart: f32,
        inputEnd: f32,
        outputStart: f32,
        outputEnd: f32,
    ) -> f32;
}
unsafe extern "C" {
    pub fn Wrap(value: f32, min: f32, max: f32) -> f32;
}
unsafe extern "C" {
    pub fn FloatEquals(x: f32, y: f32) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Vector2Zero() -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2One() -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2Add(v1: Vector2, v2: Vector2) -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2AddValue(v: Vector2, add: f32) -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2Subtract(v1: Vector2, v2: Vector2) -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2SubtractValue(v: Vector2, sub: f32) -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2Length(v: Vector2) -> f32;
}
unsafe extern "C" {
    pub fn Vector2LengthSqr(v: Vector2) -> f32;
}
unsafe extern "C" {
    pub fn Vector2DotProduct(v1: Vector2, v2: Vector2) -> f32;
}
unsafe extern "C" {
    pub fn Vector2Distance(v1: Vector2, v2: Vector2) -> f32;
}
unsafe extern "C" {
    pub fn Vector2DistanceSqr(v1: Vector2, v2: Vector2) -> f32;
}
unsafe extern "C" {
    pub fn Vector2Angle(v1: Vector2, v2: Vector2) -> f32;
}
unsafe extern "C" {
    pub fn Vector2LineAngle(start: Vector2, end: Vector2) -> f32;
}
unsafe extern "C" {
    pub fn Vector2Scale(v: Vector2, scale: f32) -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2Multiply(v1: Vector2, v2: Vector2) -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2Negate(v: Vector2) -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2Divide(v1: Vector2, v2: Vector2) -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2Normalize(v: Vector2) -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2Transform(v: Vector2, mat: Matrix) -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2Lerp(v1: Vector2, v2: Vector2, amount: f32) -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2Reflect(v: Vector2, normal: Vector2) -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2Min(v1: Vector2, v2: Vector2) -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2Max(v1: Vector2, v2: Vector2) -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2Rotate(v: Vector2, angle: f32) -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2MoveTowards(v: Vector2, target: Vector2, maxDistance: f32) -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2Invert(v: Vector2) -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2Clamp(v: Vector2, min: Vector2, max: Vector2) -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2ClampValue(v: Vector2, min: f32, max: f32) -> Vector2;
}
unsafe extern "C" {
    pub fn Vector2Equals(p: Vector2, q: Vector2) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Vector2Refract(v: Vector2, n: Vector2, r: f32) -> Vector2;
}
unsafe extern "C" {
    pub fn Vector3Zero() -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3One() -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3Add(v1: Vector3, v2: Vector3) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3AddValue(v: Vector3, add: f32) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3Subtract(v1: Vector3, v2: Vector3) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3SubtractValue(v: Vector3, sub: f32) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3Scale(v: Vector3, scalar: f32) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3Multiply(v1: Vector3, v2: Vector3) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3CrossProduct(v1: Vector3, v2: Vector3) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3Perpendicular(v: Vector3) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3Length(v: Vector3) -> f32;
}
unsafe extern "C" {
    pub fn Vector3LengthSqr(v: Vector3) -> f32;
}
unsafe extern "C" {
    pub fn Vector3DotProduct(v1: Vector3, v2: Vector3) -> f32;
}
unsafe extern "C" {
    pub fn Vector3Distance(v1: Vector3, v2: Vector3) -> f32;
}
unsafe extern "C" {
    pub fn Vector3DistanceSqr(v1: Vector3, v2: Vector3) -> f32;
}
unsafe extern "C" {
    pub fn Vector3Angle(v1: Vector3, v2: Vector3) -> f32;
}
unsafe extern "C" {
    pub fn Vector3Negate(v: Vector3) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3Divide(v1: Vector3, v2: Vector3) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3Normalize(v: Vector3) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3Project(v1: Vector3, v2: Vector3) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3Reject(v1: Vector3, v2: Vector3) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3OrthoNormalize(v1: *mut Vector3, v2: *mut Vector3);
}
unsafe extern "C" {
    pub fn Vector3Transform(v: Vector3, mat: Matrix) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3RotateByQuaternion(v: Vector3, q: Quaternion) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3RotateByAxisAngle(v: Vector3, axis: Vector3, angle: f32) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3MoveTowards(v: Vector3, target: Vector3, maxDistance: f32) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3Lerp(v1: Vector3, v2: Vector3, amount: f32) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3CubicHermite(
        v1: Vector3,
        tangent1: Vector3,
        v2: Vector3,
        tangent2: Vector3,
        amount: f32,
    ) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3Reflect(v: Vector3, normal: Vector3) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3Min(v1: Vector3, v2: Vector3) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3Max(v1: Vector3, v2: Vector3) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3Barycenter(p: Vector3, a: Vector3, b: Vector3, c: Vector3) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3Unproject(source: Vector3, projection: Matrix, view: Matrix) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3ToFloatV(v: Vector3) -> float3;
}
unsafe extern "C" {
    pub fn Vector3Invert(v: Vector3) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3Clamp(v: Vector3, min: Vector3, max: Vector3) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3ClampValue(v: Vector3, min: f32, max: f32) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector3Equals(p: Vector3, q: Vector3) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Vector3Refract(v: Vector3, n: Vector3, r: f32) -> Vector3;
}
unsafe extern "C" {
    pub fn Vector4Zero() -> Vector4;
}
unsafe extern "C" {
    pub fn Vector4One() -> Vector4;
}
unsafe extern "C" {
    pub fn Vector4Add(v1: Vector4, v2: Vector4) -> Vector4;
}
unsafe extern "C" {
    pub fn Vector4AddValue(v: Vector4, add: f32) -> Vector4;
}
unsafe extern "C" {
    pub fn Vector4Subtract(v1: Vector4, v2: Vector4) -> Vector4;
}
unsafe extern "C" {
    pub fn Vector4SubtractValue(v: Vector4, add: f32) -> Vector4;
}
unsafe extern "C" {
    pub fn Vector4Length(v: Vector4) -> f32;
}
unsafe extern "C" {
    pub fn Vector4LengthSqr(v: Vector4) -> f32;
}
unsafe extern "C" {
    pub fn Vector4DotProduct(v1: Vector4, v2: Vector4) -> f32;
}
unsafe extern "C" {
    pub fn Vector4Distance(v1: Vector4, v2: Vector4) -> f32;
}
unsafe extern "C" {
    pub fn Vector4DistanceSqr(v1: Vector4, v2: Vector4) -> f32;
}
unsafe extern "C" {
    pub fn Vector4Scale(v: Vector4, scale: f32) -> Vector4;
}
unsafe extern "C" {
    pub fn Vector4Multiply(v1: Vector4, v2: Vector4) -> Vector4;
}
unsafe extern "C" {
    pub fn Vector4Negate(v: Vector4) -> Vector4;
}
unsafe extern "C" {
    pub fn Vector4Divide(v1: Vector4, v2: Vector4) -> Vector4;
}
unsafe extern "C" {
    pub fn Vector4Normalize(v: Vector4) -> Vector4;
}
unsafe extern "C" {
    pub fn Vector4Min(v1: Vector4, v2: Vector4) -> Vector4;
}
unsafe extern "C" {
    pub fn Vector4Max(v1: Vector4, v2: Vector4) -> Vector4;
}
unsafe extern "C" {
    pub fn Vector4Lerp(v1: Vector4, v2: Vector4, amount: f32) -> Vector4;
}
unsafe extern "C" {
    pub fn Vector4MoveTowards(v: Vector4, target: Vector4, maxDistance: f32) -> Vector4;
}
unsafe extern "C" {
    pub fn Vector4Invert(v: Vector4) -> Vector4;
}
unsafe extern "C" {
    pub fn Vector4Equals(p: Vector4, q: Vector4) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn MatrixDeterminant(mat: Matrix) -> f32;
}
unsafe extern "C" {
    pub fn MatrixTrace(mat: Matrix) -> f32;
}
unsafe extern "C" {
    pub fn MatrixTranspose(mat: Matrix) -> Matrix;
}
unsafe extern "C" {
    pub fn MatrixInvert(mat: Matrix) -> Matrix;
}
unsafe extern "C" {
    pub fn MatrixIdentity() -> Matrix;
}
unsafe extern "C" {
    pub fn MatrixAdd(left: Matrix, right: Matrix) -> Matrix;
}
unsafe extern "C" {
    pub fn MatrixSubtract(left: Matrix, right: Matrix) -> Matrix;
}
unsafe extern "C" {
    pub fn MatrixMultiply(left: Matrix, right: Matrix) -> Matrix;
}
unsafe extern "C" {
    pub fn MatrixTranslate(x: f32, y: f32, z: f32) -> Matrix;
}
unsafe extern "C" {
    pub fn MatrixRotate(axis: Vector3, angle: f32) -> Matrix;
}
unsafe extern "C" {
    pub fn MatrixRotateX(angle: f32) -> Matrix;
}
unsafe extern "C" {
    pub fn MatrixRotateY(angle: f32) -> Matrix;
}
unsafe extern "C" {
    pub fn MatrixRotateZ(angle: f32) -> Matrix;
}
unsafe extern "C" {
    pub fn MatrixRotateXYZ(angle: Vector3) -> Matrix;
}
unsafe extern "C" {
    pub fn MatrixRotateZYX(angle: Vector3) -> Matrix;
}
unsafe extern "C" {
    pub fn MatrixScale(x: f32, y: f32, z: f32) -> Matrix;
}
unsafe extern "C" {
    pub fn MatrixFrustum(
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        nearPlane: f64,
        farPlane: f64,
    ) -> Matrix;
}
unsafe extern "C" {
    pub fn MatrixPerspective(fovY: f64, aspect: f64, nearPlane: f64, farPlane: f64) -> Matrix;
}
unsafe extern "C" {
    pub fn MatrixOrtho(
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        nearPlane: f64,
        farPlane: f64,
    ) -> Matrix;
}
unsafe extern "C" {
    pub fn MatrixLookAt(eye: Vector3, target: Vector3, up: Vector3) -> Matrix;
}
unsafe extern "C" {
    pub fn MatrixToFloatV(mat: Matrix) -> float16;
}
unsafe extern "C" {
    pub fn QuaternionAdd(q1: Quaternion, q2: Quaternion) -> Quaternion;
}
unsafe extern "C" {
    pub fn QuaternionAddValue(q: Quaternion, add: f32) -> Quaternion;
}
unsafe extern "C" {
    pub fn QuaternionSubtract(q1: Quaternion, q2: Quaternion) -> Quaternion;
}
unsafe extern "C" {
    pub fn QuaternionSubtractValue(q: Quaternion, sub: f32) -> Quaternion;
}
unsafe extern "C" {
    pub fn QuaternionIdentity() -> Quaternion;
}
unsafe extern "C" {
    pub fn QuaternionLength(q: Quaternion) -> f32;
}
unsafe extern "C" {
    pub fn QuaternionNormalize(q: Quaternion) -> Quaternion;
}
unsafe extern "C" {
    pub fn QuaternionInvert(q: Quaternion) -> Quaternion;
}
unsafe extern "C" {
    pub fn QuaternionMultiply(q1: Quaternion, q2: Quaternion) -> Quaternion;
}
unsafe extern "C" {
    pub fn QuaternionScale(q: Quaternion, mul: f32) -> Quaternion;
}
unsafe extern "C" {
    pub fn QuaternionDivide(q1: Quaternion, q2: Quaternion) -> Quaternion;
}
unsafe extern "C" {
    pub fn QuaternionLerp(q1: Quaternion, q2: Quaternion, amount: f32) -> Quaternion;
}
unsafe extern "C" {
    pub fn QuaternionNlerp(q1: Quaternion, q2: Quaternion, amount: f32) -> Quaternion;
}
unsafe extern "C" {
    pub fn QuaternionSlerp(q1: Quaternion, q2: Quaternion, amount: f32) -> Quaternion;
}
unsafe extern "C" {
    pub fn QuaternionCubicHermiteSpline(
        q1: Quaternion,
        outTangent1: Quaternion,
        q2: Quaternion,
        inTangent2: Quaternion,
        t: f32,
    ) -> Quaternion;
}
unsafe extern "C" {
    pub fn QuaternionFromVector3ToVector3(from: Vector3, to: Vector3) -> Quaternion;
}
unsafe extern "C" {
    pub fn QuaternionFromMatrix(mat: Matrix) -> Quaternion;
}
unsafe extern "C" {
    pub fn QuaternionToMatrix(q: Quaternion) -> Matrix;
}
unsafe extern "C" {
    pub fn QuaternionFromAxisAngle(axis: Vector3, angle: f32) -> Quaternion;
}
unsafe extern "C" {
    pub fn QuaternionToAxisAngle(q: Quaternion, outAxis: *mut Vector3, outAngle: *mut f32);
}
unsafe extern "C" {
    pub fn QuaternionFromEuler(pitch: f32, yaw: f32, roll: f32) -> Quaternion;
}
unsafe extern "C" {
    pub fn QuaternionToEuler(q: Quaternion) -> Vector3;
}
unsafe extern "C" {
    pub fn QuaternionTransform(q: Quaternion, mat: Matrix) -> Quaternion;
}
unsafe extern "C" {
    pub fn QuaternionEquals(p: Quaternion, q: Quaternion) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn MatrixDecompose(
        mat: Matrix,
        translation: *mut Vector3,
        rotation: *mut Quaternion,
        scale: *mut Vector3,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rlVertexBuffer {
    pub elementCount: ::std::os::raw::c_int,
    pub vertices: *mut f32,
    pub texcoords: *mut f32,
    pub normals: *mut f32,
    pub colors: *mut ::std::os::raw::c_uchar,
    pub indices: *mut ::std::os::raw::c_uint,
    pub vaoId: ::std::os::raw::c_uint,
    pub vboId: [::std::os::raw::c_uint; 5usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rlDrawCall {
    pub mode: ::std::os::raw::c_int,
    pub vertexCount: ::std::os::raw::c_int,
    pub vertexAlignment: ::std::os::raw::c_int,
    pub textureId: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rlRenderBatch {
    pub bufferCount: ::std::os::raw::c_int,
    pub currentBuffer: ::std::os::raw::c_int,
    pub vertexBuffer: *mut rlVertexBuffer,
    pub draws: *mut rlDrawCall,
    pub drawCounter: ::std::os::raw::c_int,
    pub currentDepth: f32,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rlGlVersion {
    RL_OPENGL_11 = 1,
    RL_OPENGL_21 = 2,
    RL_OPENGL_33 = 3,
    RL_OPENGL_43 = 4,
    RL_OPENGL_ES_20 = 5,
    RL_OPENGL_ES_30 = 6,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rlTraceLogLevel {
    RL_LOG_ALL = 0,
    RL_LOG_TRACE = 1,
    RL_LOG_DEBUG = 2,
    RL_LOG_INFO = 3,
    RL_LOG_WARNING = 4,
    RL_LOG_ERROR = 5,
    RL_LOG_FATAL = 6,
    RL_LOG_NONE = 7,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rlPixelFormat {
    RL_PIXELFORMAT_UNCOMPRESSED_GRAYSCALE = 1,
    RL_PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA = 2,
    RL_PIXELFORMAT_UNCOMPRESSED_R5G6B5 = 3,
    RL_PIXELFORMAT_UNCOMPRESSED_R8G8B8 = 4,
    RL_PIXELFORMAT_UNCOMPRESSED_R5G5B5A1 = 5,
    RL_PIXELFORMAT_UNCOMPRESSED_R4G4B4A4 = 6,
    RL_PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 = 7,
    RL_PIXELFORMAT_UNCOMPRESSED_R32 = 8,
    RL_PIXELFORMAT_UNCOMPRESSED_R32G32B32 = 9,
    RL_PIXELFORMAT_UNCOMPRESSED_R32G32B32A32 = 10,
    RL_PIXELFORMAT_UNCOMPRESSED_R16 = 11,
    RL_PIXELFORMAT_UNCOMPRESSED_R16G16B16 = 12,
    RL_PIXELFORMAT_UNCOMPRESSED_R16G16B16A16 = 13,
    RL_PIXELFORMAT_COMPRESSED_DXT1_RGB = 14,
    RL_PIXELFORMAT_COMPRESSED_DXT1_RGBA = 15,
    RL_PIXELFORMAT_COMPRESSED_DXT3_RGBA = 16,
    RL_PIXELFORMAT_COMPRESSED_DXT5_RGBA = 17,
    RL_PIXELFORMAT_COMPRESSED_ETC1_RGB = 18,
    RL_PIXELFORMAT_COMPRESSED_ETC2_RGB = 19,
    RL_PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA = 20,
    RL_PIXELFORMAT_COMPRESSED_PVRT_RGB = 21,
    RL_PIXELFORMAT_COMPRESSED_PVRT_RGBA = 22,
    RL_PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA = 23,
    RL_PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA = 24,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rlTextureFilter {
    RL_TEXTURE_FILTER_POINT = 0,
    RL_TEXTURE_FILTER_BILINEAR = 1,
    RL_TEXTURE_FILTER_TRILINEAR = 2,
    RL_TEXTURE_FILTER_ANISOTROPIC_4X = 3,
    RL_TEXTURE_FILTER_ANISOTROPIC_8X = 4,
    RL_TEXTURE_FILTER_ANISOTROPIC_16X = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rlBlendMode {
    RL_BLEND_ALPHA = 0,
    RL_BLEND_ADDITIVE = 1,
    RL_BLEND_MULTIPLIED = 2,
    RL_BLEND_ADD_COLORS = 3,
    RL_BLEND_SUBTRACT_COLORS = 4,
    RL_BLEND_ALPHA_PREMULTIPLY = 5,
    RL_BLEND_CUSTOM = 6,
    RL_BLEND_CUSTOM_SEPARATE = 7,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rlShaderLocationIndex {
    RL_SHADER_LOC_VERTEX_POSITION = 0,
    RL_SHADER_LOC_VERTEX_TEXCOORD01 = 1,
    RL_SHADER_LOC_VERTEX_TEXCOORD02 = 2,
    RL_SHADER_LOC_VERTEX_NORMAL = 3,
    RL_SHADER_LOC_VERTEX_TANGENT = 4,
    RL_SHADER_LOC_VERTEX_COLOR = 5,
    RL_SHADER_LOC_MATRIX_MVP = 6,
    RL_SHADER_LOC_MATRIX_VIEW = 7,
    RL_SHADER_LOC_MATRIX_PROJECTION = 8,
    RL_SHADER_LOC_MATRIX_MODEL = 9,
    RL_SHADER_LOC_MATRIX_NORMAL = 10,
    RL_SHADER_LOC_VECTOR_VIEW = 11,
    RL_SHADER_LOC_COLOR_DIFFUSE = 12,
    RL_SHADER_LOC_COLOR_SPECULAR = 13,
    RL_SHADER_LOC_COLOR_AMBIENT = 14,
    RL_SHADER_LOC_MAP_ALBEDO = 15,
    RL_SHADER_LOC_MAP_METALNESS = 16,
    RL_SHADER_LOC_MAP_NORMAL = 17,
    RL_SHADER_LOC_MAP_ROUGHNESS = 18,
    RL_SHADER_LOC_MAP_OCCLUSION = 19,
    RL_SHADER_LOC_MAP_EMISSION = 20,
    RL_SHADER_LOC_MAP_HEIGHT = 21,
    RL_SHADER_LOC_MAP_CUBEMAP = 22,
    RL_SHADER_LOC_MAP_IRRADIANCE = 23,
    RL_SHADER_LOC_MAP_PREFILTER = 24,
    RL_SHADER_LOC_MAP_BRDF = 25,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rlShaderUniformDataType {
    RL_SHADER_UNIFORM_FLOAT = 0,
    RL_SHADER_UNIFORM_VEC2 = 1,
    RL_SHADER_UNIFORM_VEC3 = 2,
    RL_SHADER_UNIFORM_VEC4 = 3,
    RL_SHADER_UNIFORM_INT = 4,
    RL_SHADER_UNIFORM_IVEC2 = 5,
    RL_SHADER_UNIFORM_IVEC3 = 6,
    RL_SHADER_UNIFORM_IVEC4 = 7,
    RL_SHADER_UNIFORM_UINT = 8,
    RL_SHADER_UNIFORM_UIVEC2 = 9,
    RL_SHADER_UNIFORM_UIVEC3 = 10,
    RL_SHADER_UNIFORM_UIVEC4 = 11,
    RL_SHADER_UNIFORM_SAMPLER2D = 12,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rlShaderAttributeDataType {
    RL_SHADER_ATTRIB_FLOAT = 0,
    RL_SHADER_ATTRIB_VEC2 = 1,
    RL_SHADER_ATTRIB_VEC3 = 2,
    RL_SHADER_ATTRIB_VEC4 = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rlFramebufferAttachType {
    RL_ATTACHMENT_COLOR_CHANNEL0 = 0,
    RL_ATTACHMENT_COLOR_CHANNEL1 = 1,
    RL_ATTACHMENT_COLOR_CHANNEL2 = 2,
    RL_ATTACHMENT_COLOR_CHANNEL3 = 3,
    RL_ATTACHMENT_COLOR_CHANNEL4 = 4,
    RL_ATTACHMENT_COLOR_CHANNEL5 = 5,
    RL_ATTACHMENT_COLOR_CHANNEL6 = 6,
    RL_ATTACHMENT_COLOR_CHANNEL7 = 7,
    RL_ATTACHMENT_DEPTH = 100,
    RL_ATTACHMENT_STENCIL = 200,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rlFramebufferAttachTextureType {
    RL_ATTACHMENT_CUBEMAP_POSITIVE_X = 0,
    RL_ATTACHMENT_CUBEMAP_NEGATIVE_X = 1,
    RL_ATTACHMENT_CUBEMAP_POSITIVE_Y = 2,
    RL_ATTACHMENT_CUBEMAP_NEGATIVE_Y = 3,
    RL_ATTACHMENT_CUBEMAP_POSITIVE_Z = 4,
    RL_ATTACHMENT_CUBEMAP_NEGATIVE_Z = 5,
    RL_ATTACHMENT_TEXTURE2D = 100,
    RL_ATTACHMENT_RENDERBUFFER = 200,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rlCullMode {
    RL_CULL_FACE_FRONT = 0,
    RL_CULL_FACE_BACK = 1,
}
unsafe extern "C" {
    pub fn rlMatrixMode(mode: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn rlPushMatrix();
}
unsafe extern "C" {
    pub fn rlPopMatrix();
}
unsafe extern "C" {
    pub fn rlLoadIdentity();
}
unsafe extern "C" {
    pub fn rlTranslatef(x: f32, y: f32, z: f32);
}
unsafe extern "C" {
    pub fn rlRotatef(angle: f32, x: f32, y: f32, z: f32);
}
unsafe extern "C" {
    pub fn rlScalef(x: f32, y: f32, z: f32);
}
unsafe extern "C" {
    pub fn rlMultMatrixf(matf: *const f32);
}
unsafe extern "C" {
    pub fn rlFrustum(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64);
}
unsafe extern "C" {
    pub fn rlOrtho(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64);
}
unsafe extern "C" {
    pub fn rlViewport(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn rlSetClipPlanes(nearPlane: f64, farPlane: f64);
}
unsafe extern "C" {
    pub fn rlGetCullDistanceNear() -> f64;
}
unsafe extern "C" {
    pub fn rlGetCullDistanceFar() -> f64;
}
unsafe extern "C" {
    pub fn rlBegin(mode: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn rlEnd();
}
unsafe extern "C" {
    pub fn rlVertex2i(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn rlVertex2f(x: f32, y: f32);
}
unsafe extern "C" {
    pub fn rlVertex3f(x: f32, y: f32, z: f32);
}
unsafe extern "C" {
    pub fn rlTexCoord2f(x: f32, y: f32);
}
unsafe extern "C" {
    pub fn rlNormal3f(x: f32, y: f32, z: f32);
}
unsafe extern "C" {
    pub fn rlColor4ub(
        r: ::std::os::raw::c_uchar,
        g: ::std::os::raw::c_uchar,
        b: ::std::os::raw::c_uchar,
        a: ::std::os::raw::c_uchar,
    );
}
unsafe extern "C" {
    pub fn rlColor3f(x: f32, y: f32, z: f32);
}
unsafe extern "C" {
    pub fn rlColor4f(x: f32, y: f32, z: f32, w: f32);
}
unsafe extern "C" {
    pub fn rlEnableVertexArray(vaoId: ::std::os::raw::c_uint) -> bool;
}
unsafe extern "C" {
    pub fn rlDisableVertexArray();
}
unsafe extern "C" {
    pub fn rlEnableVertexBuffer(id: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn rlDisableVertexBuffer();
}
unsafe extern "C" {
    pub fn rlEnableVertexBufferElement(id: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn rlDisableVertexBufferElement();
}
unsafe extern "C" {
    pub fn rlEnableVertexAttribute(index: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn rlDisableVertexAttribute(index: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn rlActiveTextureSlot(slot: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn rlEnableTexture(id: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn rlDisableTexture();
}
unsafe extern "C" {
    pub fn rlEnableTextureCubemap(id: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn rlDisableTextureCubemap();
}
unsafe extern "C" {
    pub fn rlTextureParameters(
        id: ::std::os::raw::c_uint,
        param: ::std::os::raw::c_int,
        value: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn rlCubemapParameters(
        id: ::std::os::raw::c_uint,
        param: ::std::os::raw::c_int,
        value: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn rlEnableShader(id: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn rlDisableShader();
}
unsafe extern "C" {
    pub fn rlEnableFramebuffer(id: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn rlDisableFramebuffer();
}
unsafe extern "C" {
    pub fn rlGetActiveFramebuffer() -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn rlActiveDrawBuffers(count: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn rlBlitFramebuffer(
        srcX: ::std::os::raw::c_int,
        srcY: ::std::os::raw::c_int,
        srcWidth: ::std::os::raw::c_int,
        srcHeight: ::std::os::raw::c_int,
        dstX: ::std::os::raw::c_int,
        dstY: ::std::os::raw::c_int,
        dstWidth: ::std::os::raw::c_int,
        dstHeight: ::std::os::raw::c_int,
        bufferMask: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn rlBindFramebuffer(target: ::std::os::raw::c_uint, framebuffer: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn rlEnableColorBlend();
}
unsafe extern "C" {
    pub fn rlDisableColorBlend();
}
unsafe extern "C" {
    pub fn rlEnableDepthTest();
}
unsafe extern "C" {
    pub fn rlDisableDepthTest();
}
unsafe extern "C" {
    pub fn rlEnableDepthMask();
}
unsafe extern "C" {
    pub fn rlDisableDepthMask();
}
unsafe extern "C" {
    pub fn rlEnableBackfaceCulling();
}
unsafe extern "C" {
    pub fn rlDisableBackfaceCulling();
}
unsafe extern "C" {
    pub fn rlColorMask(r: bool, g: bool, b: bool, a: bool);
}
unsafe extern "C" {
    pub fn rlSetCullFace(mode: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn rlEnableScissorTest();
}
unsafe extern "C" {
    pub fn rlDisableScissorTest();
}
unsafe extern "C" {
    pub fn rlScissor(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn rlEnableWireMode();
}
unsafe extern "C" {
    pub fn rlEnablePointMode();
}
unsafe extern "C" {
    pub fn rlDisableWireMode();
}
unsafe extern "C" {
    pub fn rlSetLineWidth(width: f32);
}
unsafe extern "C" {
    pub fn rlGetLineWidth() -> f32;
}
unsafe extern "C" {
    pub fn rlEnableSmoothLines();
}
unsafe extern "C" {
    pub fn rlDisableSmoothLines();
}
unsafe extern "C" {
    pub fn rlEnableStereoRender();
}
unsafe extern "C" {
    pub fn rlDisableStereoRender();
}
unsafe extern "C" {
    pub fn rlIsStereoRenderEnabled() -> bool;
}
unsafe extern "C" {
    pub fn rlClearColor(
        r: ::std::os::raw::c_uchar,
        g: ::std::os::raw::c_uchar,
        b: ::std::os::raw::c_uchar,
        a: ::std::os::raw::c_uchar,
    );
}
unsafe extern "C" {
    pub fn rlClearScreenBuffers();
}
unsafe extern "C" {
    pub fn rlCheckErrors();
}
unsafe extern "C" {
    pub fn rlSetBlendMode(mode: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn rlSetBlendFactors(
        glSrcFactor: ::std::os::raw::c_int,
        glDstFactor: ::std::os::raw::c_int,
        glEquation: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn rlSetBlendFactorsSeparate(
        glSrcRGB: ::std::os::raw::c_int,
        glDstRGB: ::std::os::raw::c_int,
        glSrcAlpha: ::std::os::raw::c_int,
        glDstAlpha: ::std::os::raw::c_int,
        glEqRGB: ::std::os::raw::c_int,
        glEqAlpha: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn rlglInit(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn rlglClose();
}
unsafe extern "C" {
    pub fn rlLoadExtensions(loader: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn rlGetVersion() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn rlSetFramebufferWidth(width: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn rlGetFramebufferWidth() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn rlSetFramebufferHeight(height: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn rlGetFramebufferHeight() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn rlGetTextureIdDefault() -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn rlGetShaderIdDefault() -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn rlGetShaderLocsDefault() -> *mut ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn rlLoadRenderBatch(
        numBuffers: ::std::os::raw::c_int,
        bufferElements: ::std::os::raw::c_int,
    ) -> rlRenderBatch;
}
unsafe extern "C" {
    pub fn rlUnloadRenderBatch(batch: rlRenderBatch);
}
unsafe extern "C" {
    pub fn rlDrawRenderBatch(batch: *mut rlRenderBatch);
}
unsafe extern "C" {
    pub fn rlSetRenderBatchActive(batch: *mut rlRenderBatch);
}
unsafe extern "C" {
    pub fn rlDrawRenderBatchActive();
}
unsafe extern "C" {
    pub fn rlCheckRenderBatchLimit(vCount: ::std::os::raw::c_int) -> bool;
}
unsafe extern "C" {
    pub fn rlSetTexture(id: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn rlLoadVertexArray() -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn rlLoadVertexBuffer(
        buffer: *const ::std::os::raw::c_void,
        size: ::std::os::raw::c_int,
        dynamic: bool,
    ) -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn rlLoadVertexBufferElement(
        buffer: *const ::std::os::raw::c_void,
        size: ::std::os::raw::c_int,
        dynamic: bool,
    ) -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn rlUpdateVertexBuffer(
        bufferId: ::std::os::raw::c_uint,
        data: *const ::std::os::raw::c_void,
        dataSize: ::std::os::raw::c_int,
        offset: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn rlUpdateVertexBufferElements(
        id: ::std::os::raw::c_uint,
        data: *const ::std::os::raw::c_void,
        dataSize: ::std::os::raw::c_int,
        offset: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn rlUnloadVertexArray(vaoId: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn rlUnloadVertexBuffer(vboId: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn rlSetVertexAttribute(
        index: ::std::os::raw::c_uint,
        compSize: ::std::os::raw::c_int,
        type_: ::std::os::raw::c_int,
        normalized: bool,
        stride: ::std::os::raw::c_int,
        offset: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn rlSetVertexAttributeDivisor(
        index: ::std::os::raw::c_uint,
        divisor: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn rlSetVertexAttributeDefault(
        locIndex: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_void,
        attribType: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn rlDrawVertexArray(offset: ::std::os::raw::c_int, count: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn rlDrawVertexArrayElements(
        offset: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
        buffer: *const ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn rlDrawVertexArrayInstanced(
        offset: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
        instances: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn rlDrawVertexArrayElementsInstanced(
        offset: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
        buffer: *const ::std::os::raw::c_void,
        instances: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn rlLoadTexture(
        data: *const ::std::os::raw::c_void,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
        mipmapCount: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn rlLoadTextureDepth(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        useRenderBuffer: bool,
    ) -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn rlLoadTextureCubemap(
        data: *const ::std::os::raw::c_void,
        size: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
        mipmapCount: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn rlUpdateTexture(
        id: ::std::os::raw::c_uint,
        offsetX: ::std::os::raw::c_int,
        offsetY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
        data: *const ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn rlGetGlTextureFormats(
        format: ::std::os::raw::c_int,
        glInternalFormat: *mut ::std::os::raw::c_uint,
        glFormat: *mut ::std::os::raw::c_uint,
        glType: *mut ::std::os::raw::c_uint,
    );
}
unsafe extern "C" {
    pub fn rlGetPixelFormatName(format: ::std::os::raw::c_uint) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn rlUnloadTexture(id: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn rlGenTextureMipmaps(
        id: ::std::os::raw::c_uint,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
        mipmaps: *mut ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn rlReadTexturePixels(
        id: ::std::os::raw::c_uint,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn rlReadScreenPixels(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
unsafe extern "C" {
    pub fn rlLoadFramebuffer() -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn rlFramebufferAttach(
        fboId: ::std::os::raw::c_uint,
        texId: ::std::os::raw::c_uint,
        attachType: ::std::os::raw::c_int,
        texType: ::std::os::raw::c_int,
        mipLevel: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn rlFramebufferComplete(id: ::std::os::raw::c_uint) -> bool;
}
unsafe extern "C" {
    pub fn rlUnloadFramebuffer(id: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn rlLoadShaderCode(
        vsCode: *const ::std::os::raw::c_char,
        fsCode: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn rlCompileShader(
        shaderCode: *const ::std::os::raw::c_char,
        type_: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn rlLoadShaderProgram(
        vShaderId: ::std::os::raw::c_uint,
        fShaderId: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn rlUnloadShaderProgram(id: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn rlGetLocationUniform(
        shaderId: ::std::os::raw::c_uint,
        uniformName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn rlGetLocationAttrib(
        shaderId: ::std::os::raw::c_uint,
        attribName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn rlSetUniform(
        locIndex: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_void,
        uniformType: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn rlSetUniformMatrix(locIndex: ::std::os::raw::c_int, mat: Matrix);
}
unsafe extern "C" {
    pub fn rlSetUniformMatrices(
        locIndex: ::std::os::raw::c_int,
        mat: *const Matrix,
        count: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn rlSetUniformSampler(locIndex: ::std::os::raw::c_int, textureId: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn rlSetShader(id: ::std::os::raw::c_uint, locs: *mut ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn rlLoadComputeShaderProgram(shaderId: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn rlComputeShaderDispatch(
        groupX: ::std::os::raw::c_uint,
        groupY: ::std::os::raw::c_uint,
        groupZ: ::std::os::raw::c_uint,
    );
}
unsafe extern "C" {
    pub fn rlLoadShaderBuffer(
        size: ::std::os::raw::c_uint,
        data: *const ::std::os::raw::c_void,
        usageHint: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn rlUnloadShaderBuffer(ssboId: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn rlUpdateShaderBuffer(
        id: ::std::os::raw::c_uint,
        data: *const ::std::os::raw::c_void,
        dataSize: ::std::os::raw::c_uint,
        offset: ::std::os::raw::c_uint,
    );
}
unsafe extern "C" {
    pub fn rlBindShaderBuffer(id: ::std::os::raw::c_uint, index: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn rlReadShaderBuffer(
        id: ::std::os::raw::c_uint,
        dest: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_uint,
        offset: ::std::os::raw::c_uint,
    );
}
unsafe extern "C" {
    pub fn rlCopyShaderBuffer(
        destId: ::std::os::raw::c_uint,
        srcId: ::std::os::raw::c_uint,
        destOffset: ::std::os::raw::c_uint,
        srcOffset: ::std::os::raw::c_uint,
        count: ::std::os::raw::c_uint,
    );
}
unsafe extern "C" {
    pub fn rlGetShaderBufferSize(id: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn rlBindImageTexture(
        id: ::std::os::raw::c_uint,
        index: ::std::os::raw::c_uint,
        format: ::std::os::raw::c_int,
        readonly: bool,
    );
}
unsafe extern "C" {
    pub fn rlGetMatrixModelview() -> Matrix;
}
unsafe extern "C" {
    pub fn rlGetMatrixProjection() -> Matrix;
}
unsafe extern "C" {
    pub fn rlGetMatrixTransform() -> Matrix;
}
unsafe extern "C" {
    pub fn rlGetMatrixProjectionStereo(eye: ::std::os::raw::c_int) -> Matrix;
}
unsafe extern "C" {
    pub fn rlGetMatrixViewOffsetStereo(eye: ::std::os::raw::c_int) -> Matrix;
}
unsafe extern "C" {
    pub fn rlSetMatrixProjection(proj: Matrix);
}
unsafe extern "C" {
    pub fn rlSetMatrixModelview(view: Matrix);
}
unsafe extern "C" {
    pub fn rlSetMatrixProjectionStereo(right: Matrix, left: Matrix);
}
unsafe extern "C" {
    pub fn rlSetMatrixViewOffsetStereo(right: Matrix, left: Matrix);
}
unsafe extern "C" {
    pub fn rlLoadDrawCube();
}
unsafe extern "C" {
    pub fn rlLoadDrawQuad();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
