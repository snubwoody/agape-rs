@group(0) @binding(0) var<uniform> window_size: vec2<f32>;

struct VertexOutput{
	@builtin(position) position: vec4<f32>,
	@location(0) color: vec4<f32>
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


@vertex
fn vs_main(model:VertexInput) -> VertexOutput {
	var out: VertexOutput;
	out.color = model.color;
	
	// Normalize the coordinates
	var x_pos = normalize_value(model.position.x,0.0,window_size.x);
	var y_pos = -normalize_value(model.position.y,0.0,window_size.y);
	
	out.position = vec4<f32>(x_pos,y_pos,1.0,1.0);
	return out;
}

@fragment
fn fs_main(in:VertexOutput) -> @location(0) vec4<f32> {
	var x_pos = normalize_value(in.position.x,0.0,window_size.x);
	var y_pos = normalize_value(in.position.y,0.0,window_size.y);
	var radius = distance(vec2(x_pos,y_pos),vec2(0.5,0.5));
	return vec4(radius,radius,1.0,1.0);
}