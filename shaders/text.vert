#version 410 

in vec2 position;
in vec2 tex_coords;

out vec4 vertexColour;
out vec2 texCoord;

void main() {
	vertexColour = vec4(1.0,0.4,0.6,1.0);
	texCoord = tex_coords;
    gl_Position = vec4(vecPosition, 0.0,1.0);
}
