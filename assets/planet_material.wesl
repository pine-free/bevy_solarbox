import bevy_pbr::render::forward_io::VertexOutput;

struct Material {
    color: vec3f,
    radius: f32,
    position: vec2f,
}

@group(2) @binding(1)
var<uniform> material: Material;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var result = vec4f(1.);
    if length(in.world_position.xy - material.position) <= material.radius {
        result = vec4f(material.color, 1.0);
    } else {
        discard;
    }
    return result;
}
