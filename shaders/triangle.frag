#version 410
out vec4 fragColour;

in vec4 vertexColour;


void main() {
	fragColour = vertexColour;
}