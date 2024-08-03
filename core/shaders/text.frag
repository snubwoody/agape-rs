#version 410
out vec4 fragColour;

in vec4 vertexColour;
in vec2 texCoord;

uniform sampler2D tex;


void main() {
	fragColour = texture(tex,texCoord);
}