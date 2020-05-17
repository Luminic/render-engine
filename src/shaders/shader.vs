#version 450

layout (location=0) in vec2 v_position;
layout (location=1) in int v_texture_binding;
layout (location=2) in vec4 v_tex_coords_or_color;

layout (location=0) out VS_OUT {
    flat int texture_binding;
    vec4 tex_coords_or_color;
} vs_out;


layout(set=0, binding=0) uniform Uniforms {
    // A vec3 must be aligned to a vec4
    // A mat3 is just 3 vec3s
    // however, when sending data to the shader, no padding is given and the shader doesn't compensate for this resulting in malformed mat3s
    // So I'm avoiding the problem by manually padding out my mat3s to mat4 and converting them back
    // When this issue is fixed feel free to replace the mat4 with mat3 (and corresponding mat4 in uniforms)
    // see: https://github.com/gfx-rs/wgpu-rs/issues/36 and https://stackoverflow.com/questions/61768628/wgpu-rs-putting-a-matrix3-into-a-vertex-shader-results-in-odd-behavior-but-usin/61779788#61779788
    mat4 camera_transform;
};

void main () {
    gl_Position = vec4((mat3(camera_transform)*vec3(v_position,1.0f)).xy, 0.0f, 1.0f);
    vs_out.texture_binding = v_texture_binding;
    vs_out.tex_coords_or_color = v_tex_coords_or_color;
}