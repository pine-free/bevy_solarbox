import bevy_pbr::render::forward_io::VertexOutput;

struct Material {
    orbit_data: array<vec3, 128>,
}

@group(2) @binding(0)
var<uniform> material: Material;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var result = vec3<f32>(0.0, 0.0, 0.0);

    ///

    return vec4<f32>(result, 1.0);
}

