// Window attributes
@group(0) @binding(0) var<uniform> window_size: vec2<f32>;

// Widget attributes
@group(1) @binding(0) var<uniform> center_pos: vec2<f32>;
@group(1) @binding(1) var<uniform> size: vec2<f32>;
@group(1) @binding(2) var<uniform> position: vec2<f32>;

struct VertexOutput{
	@builtin(position) position: vec4<f32>,
	@location(0) color: vec4<f32>,
	@location(1) uv:vec2<f32>
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

// Rounded box SDF
fn roundedBoxSdf(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + vec2<f32>(r);
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - r;
}

// Convert screen space coordinates to normalised device coordinates
fn screen_to_ndc(in:vec2<f32>) -> vec2<f32>{
	return vec2<f32>(
		(in.x / window_size.x) * 2.0 - 1.0,
		-((in.y / window_size.y) * 2.0 - 1.0),
	);
}

@vertex
fn vs_main(in:VertexInput) -> VertexOutput {
	var out: VertexOutput;
	out.color = in.color;
	
	// Normalize the coordinates
	var x_pos = normalize_value(in.position.x,0.0,window_size.x);
	var y_pos = -normalize_value(in.position.y,0.0,window_size.y);
	var coords =  screen_to_ndc(in.position);
	
	out.position = vec4<f32>(coords,1.0,1.0);
	// Store UV coordinates for fragment shader
    out.uv = in.position / window_size;
	return out;
}

@fragment
fn fs_main(in:VertexOutput) -> @location(0) vec4<f32> {
	// Convert from screen coordinates to UV coordinates centered at box center
	let center_uv = center_pos / window_size;
    let rel_pos = in.uv - center_uv;
    
    // Box dimensions (half-width and half-height)
    let box_size = (size / window_size) * 0.5; // Adjust these values for different box sizes
    
    // Corner radius
    let radius = 24/window_size.x; // Adjust this value for different corner roundness
    
    // Calculate SDF
    let d = roundedBoxSdf(rel_pos, box_size, radius);
    
    // Create sharp edges with smoothstep
    let edge = 1.0 - smoothstep(0.0, 0.005, d);
    
    // Final color
    return vec4<f32>(edge) * in.color;
}