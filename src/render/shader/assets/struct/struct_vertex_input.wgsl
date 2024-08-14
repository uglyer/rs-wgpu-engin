// #include <copyright>
struct VertexInput {
#ifdef STRUCT_VERTEX_INPUT_HAS_POSITION
    @location(0) position: vec3<f32>,
#endif
#ifdef STRUCT_VERTEX_INPUT_HAS_NORMAL
    @location(1) normal: vec3<f32>,
#endif
#ifdef STRUCT_VERTEX_INPUT_HAS_TEX_COORDS
    @location(2) tex_coords: vec3<f32>,
#endif
#ifdef STRUCT_VERTEX_INPUT_HAS_COLOR
    @location(3) color: vec3<f32>,
#endif
};
