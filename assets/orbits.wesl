import bevy_pbr::render::forward_io::VertexOutput;


const buf_size = 128;
struct Material {
    orbit_data: array<vec3f, buf_size>,
}

@group(2) @binding(0)
var<uniform> material: Material;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var result = vec4f(0.0, 0.0, 0.0, 0.0);

    for (var i = 0; i < buf_size; i++) {
        var orbit_radius = material.orbit_data[i].z;
        if orbit_radius < 0 {
                discard;
        }
        var l = length(in.world_position.xy - material.orbit_data[i].xy);
        
        if l <= orbit_radius+1 && l >= orbit_radius-1 {
                return vec4f(vec3f(1.0), 0.5);
        } 
    }

    return result;
}

