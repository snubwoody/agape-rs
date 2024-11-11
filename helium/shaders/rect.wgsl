// Window attributes
@group(0) @binding(0) var<uniform> window_size: vec2<f32>;

// Widget attributes
@group(1) @binding(0) var<uniform> center_pos: vec2<f32>;
@group(1) @binding(1) var<uniform> size: vec2<f32>;
@group(1) @binding(2) var<uniform> position: vec2<f32>;

struct VertexOutput{
	@builtin(position) position: vec4<f32>,
	@location(0) color: vec4<f32>,
}

struct VertexInput{
	@location(0) position: vec2<f32>,
	@location(1) color:vec4<f32>,
	@location(2) uv:vec2<f32>
}

// Normalize the value to a 0-1 scale.
fn normalize_value(value:f32, min_value:f32, max_value:f32) -> f32{
	var output:f32 = value;
    if (value > max_value){
		output = max_value;
	}
	
    if (value < min_value) {
		output = min_value;
	}

	var scale: f32 =  (1.0 - -1.0) / (max_value - min_value);
	var offset:f32 = min_value*(scale) + -1;
	

	return output * scale + offset;
}

// Convert screen space coordinates to normalised device coordinates
fn screen_to_ndc(in:vec2<f32>) -> vec2<f32>{
	return vec2<f32>(
		(in.x / window_size.x) * 2.0 - 1.0, // Scale by 2 and translate by -1
		-((in.y / window_size.y) * 2.0 - 1.0),
	);
}

fn roundedBoxSdf( p:vec2<f32>, b:vec2<f32>, r:vec4<f32> ) -> f32 {
    //r.xy = (p.x>0.0)?r.xy : r.zw;
	var radius = r;
	// if !(p.x > 0.0) {
	// 	radius.x = r.z;
	// 	radius.y = r.w;
	// }

	// if !(p.y > 0.0) {
	// 	radius.x = r.y;
	// }
    //r.x  = (p.y>0.0)?r.x  : r.y;
    let q = abs(p)-b+r.xy;

    return min(max(q.x,q.y),0.0) + length(max(q,vec2(0.0))) - r.x;
}

@vertex
fn vs_main(in:VertexInput) -> VertexOutput {
	var out: VertexOutput;
	
	// Normalize the coordinates
	var coords =  screen_to_ndc(in.position);
	
	out.position = vec4<f32>(coords,1.0,1.0);
	out.color = in.color;
	return out;
}

@fragment
fn fs_main(in:VertexOutput) -> @location(0) vec4<f32> {

	// Get the uv coords of the current fragment
	let uv = in.position.xy / window_size;

    let bounds = size/window_size;
	let radius = vec4(1.0);
	let p = uv - bounds;
	let d = roundedBoxSdf(p,bounds/2,radius);

    return in.color;
}