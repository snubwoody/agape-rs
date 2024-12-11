// Window attributes
@group(0) @binding(0) var<uniform> window_size: vec2<f32>;

// Widget attributes
@group(1) @binding(0) var<uniform> corner_radius: f32;
@group(1) @binding(1) var<uniform> size: vec2<f32>;
@group(1) @binding(2) var<uniform> position: vec2<f32>;

struct VertexOutput{
	@builtin(position) position: vec4<f32>,
	@location(0) color: vec4<f32>, // TODO probably don't need the color in every vertex
}

struct VertexInput{
	@location(0) position: vec2<f32>,
	@location(1) color:vec4<f32>,
	@location(2) uv:vec2<f32>
}

// Convert screen space coordinates to normalised device coordinates
fn screen_to_ndc(in:vec2<f32>) -> vec2<f32>{
	return vec2<f32>(
		(in.x / window_size.x) * 2.0 - 1.0, // Scale by 2 and translate by -1
		-((in.y / window_size.y) * 2.0 - 1.0),
	);
}

// b.x = width
// b.y = height
// r.x = roundness top-right  
// r.y = roundness bottom-right
// r.z = roundness top-left
// r.w = roundness bottom-left
fn sd_rounded_box( point:vec2<f32>,bounds:vec2<f32>, radius:vec4<f32> ) -> f32 {
	var r = radius;
	if point.x < 0.0 {
		r.x  = r.z;
		r.y  = r.w;
	}
	if point.y < 0.0 {
		r.x = r.y;
	}
    let q = abs(point)-bounds+r.x;
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
	let center = (position + (size * 0.5)); // Over 2
	let p = in.position.xy - center;

	let d = sd_rounded_box(
		p,
		size/2,
		vec4(corner_radius)
	);// Might need to clamp radius
	return vec4(in.color.xyz,-d * in.color.w);
}

// float roundedBoxSDF(vec2 CenterPosition, vec2 Size, vec4 Radius)
// {
//     Radius.xy = (CenterPosition.x > 0.0) ? Radius.xy : Radius.zw;
//     Radius.x  = (CenterPosition.y > 0.0) ? Radius.x  : Radius.y;
    
//     vec2 q = abs(CenterPosition)-Size+Radius.x;
//     return min(max(q.x,q.y),0.0) + length(max(q,0.0)) - Radius.x;
// }

// void mainImage( out vec4 fragColor, in vec2 fragCoord )
// {
//     // =========================================================================
//     // Inputs (uniforms)

//     vec2  u_rectSize   = vec2(250.0, 250.0);     // The pixel-space scale of the rectangle.
//     vec2  u_rectCenter = (iResolution.xy / 2.0); // The pixel-space rectangle center location
    
//     float u_edgeSoftness   = 10.0; // How soft the edges should be (in pixels). Higher values could be used to simulate a drop shadow.
//     vec4  u_cornerRadiuses = vec4(10.0, 20.0, 40.0, 60.0); // The radiuses of the corners(in pixels): [topRight, bottomRight, topLeft, bottomLeft]
    
//     // Border
//     float u_borderThickness = 5.0; // The border size (in pixels) 
//     float u_borderSoftness  = 0.0; // How soft the (internal) border should be (in pixels)
    
//     // Shadow
//     float u_shadowSoftness = 30.0;            // The (half) shadow radius (in pixels)
//     vec2  u_shadowOffset   = vec2(0.0, 10.0); // The pixel-space shadow offset from rectangle center
    
//     // Colors
//     vec4  u_colorBg     = vec4(0.93, 0.93, 0.93, 1.0); // The color of background
//     vec4  u_colorRect   = vec4(1.0,  0.30, 0.45, 1.0); // The color of rectangle
//     vec4  u_colorBorder = vec4(0.7,  0.25, 0.55, 1.0); // The color of (internal) border
//     vec4  u_colorShadow = vec4(0.4,  0.4,  0.4,  1.0); // The color of shadow
    
//     // =========================================================================

//     vec2 halfSize = (u_rectSize / 2.0); // Rectangle extents (half of the size)
    
//     vec4 radius = vec4((sin(iTime) + 1.0)) * u_cornerRadiuses; // Animated corners radiuses
    
//     // -------------------------------------------------------------------------
    
//     // Calculate distance to edge.   
//     float distance = roundedBoxSDF(fragCoord.xy - u_rectCenter, halfSize, radius);
       
//     // Smooth the result (free antialiasing).
//     float smoothedAlpha = 1.0-smoothstep(0.0, u_edgeSoftness, distance);
    
//     // -------------------------------------------------------------------------
//     // Border.
    
//     float borderAlpha   = 1.0-smoothstep(u_borderThickness - u_borderSoftness, u_borderThickness, abs(distance));
    
//     // -------------------------------------------------------------------------
//     // Apply a drop shadow effect.
    
//     float shadowDistance  = roundedBoxSDF(fragCoord.xy - u_rectCenter + u_shadowOffset, halfSize, radius);
//     float shadowAlpha 	  = 1.0-smoothstep(-u_shadowSoftness, u_shadowSoftness, shadowDistance);
    

//     // -------------------------------------------------------------------------
//     // Debug output
    
//         // vec4 debug_sdf = vec4(distance, 0.0, 0.0, 1.0);
    
//         // Notice, that instead simple 'alpha' here is used 'min(u_colorRect.a, alpha)' to enable transparency
//         // vec4 debug_rect_color   = mix(u_colorBg, u_colorRect, min(u_colorRect.a, smoothedAlpha));
    
//         // Notice, that instead simple 'alpha' here is used 'min(u_colorBorder.a, alpha)' to enable transparency
//         // vec4 debug_border_color = mix(u_colorBg, u_colorBorder, min(u_colorBorder.a, min(borderAlpha, smoothedAlpha)) ); 

//     // -------------------------------------------------------------------------
//     // Apply colors layer-by-layer: background <- shadow <- rect <- border.
    
//     // Blend background with shadow
//     vec4 res_shadow_color = mix(u_colorBg, vec4(u_colorShadow.rgb, shadowAlpha), shadowAlpha);

//     // Blend (background+shadow) with rect
//     //   Note:
//     //     - Used 'min(u_colorRect.a, smoothedAlpha)' instead of 'smoothedAlpha'
//     //       to enable rectangle color transparency
//     vec4 res_shadow_with_rect_color = 
//         mix(
//             res_shadow_color,
//             u_colorRect,
//             min(u_colorRect.a, smoothedAlpha)
//         );
        
//     // Blend (background+shadow+rect) with border
//     //   Note:
//     //     - Used 'min(borderAlpha, smoothedAlpha)' instead of 'borderAlpha'
//     //       to make border 'internal'
//     //     - Used 'min(u_colorBorder.a, alpha)' instead of 'alpha' to enable
//     //       border color transparency
//     vec4 res_shadow_with_rect_with_border =
//         mix(
//             res_shadow_with_rect_color,
//             u_colorBorder,
//             min(u_colorBorder.a, min(borderAlpha, smoothedAlpha))
//         );
    
//     // -------------------------------------------------------------------------
     
//     fragColor = res_shadow_with_rect_with_border;
// }


