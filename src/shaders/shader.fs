#version 450

layout (location=0) out vec4 frag_color;

layout (location=0) in VS_OUT {
    flat int texture_binding;
    vec4 tex_coords_or_color;
} fs_in;

layout(set=1, binding=0) uniform sampler s0;
layout(set=1, binding=1) uniform sampler s1;
layout(set=1, binding=2) uniform sampler s2;
layout(set=1, binding=3) uniform sampler s3;
layout(set=1, binding=4) uniform sampler s4;
layout(set=1, binding=5) uniform sampler s5;

layout (set=2, binding=0) uniform texture2D t0;
layout (set=2, binding=1) uniform texture2D t1;
layout (set=2, binding=2) uniform texture2D t2;
layout (set=2, binding=3) uniform texture2D t3;
layout (set=2, binding=4) uniform texture2D t4;
layout (set=2, binding=5) uniform texture2D t5;
layout (set=2, binding=6) uniform texture2D t6;
layout (set=2, binding=7) uniform texture2D t7;
layout (set=2, binding=8) uniform texture2D t8;
layout (set=2, binding=9) uniform texture2D t9;

void main() {
    vec4 color;
    if (fs_in.texture_binding < 0) {
        color = fs_in.tex_coords_or_color;
    } else if (fs_in.texture_binding == 0) {
        color = texture(sampler2D(t0, s3), fs_in.tex_coords_or_color.xy);
    } else if (fs_in.texture_binding == 1) {
        color = texture(sampler2D(t1, s3), fs_in.tex_coords_or_color.xy);
    } else if (fs_in.texture_binding == 2) {
        color = texture(sampler2D(t2, s3), fs_in.tex_coords_or_color.xy);
    } else if (fs_in.texture_binding == 3) {
        color = texture(sampler2D(t3, s3), fs_in.tex_coords_or_color.xy);
    } else if (fs_in.texture_binding == 4) {
        color = texture(sampler2D(t4, s3), fs_in.tex_coords_or_color.xy);
    } else if (fs_in.texture_binding == 5) {
        color = texture(sampler2D(t5, s3), fs_in.tex_coords_or_color.xy);
    } else if (fs_in.texture_binding == 6) {
        color = texture(sampler2D(t6, s3), fs_in.tex_coords_or_color.xy);
    } else if (fs_in.texture_binding == 7) {
        color = texture(sampler2D(t7, s3), fs_in.tex_coords_or_color.xy);
    } else if (fs_in.texture_binding == 8) {
        color = texture(sampler2D(t8, s3), fs_in.tex_coords_or_color.xy);
    } else if (fs_in.texture_binding == 9) {
        color = texture(sampler2D(t9, s3), fs_in.tex_coords_or_color.xy);
    } else {
        color = vec4(1.0f,0.0f,1.0f,1.0f);
    }
    frag_color = color;
}