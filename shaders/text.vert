#version 410 

layout (location = 0) in vec2 vecPosition;
layout (location = 1) in vec4 vecColour;
layout (location = 2) in vec2 texCoordinate;

out vec4 vertexColour;
out vec2 texCoord;

void main() {
	vertexColour = vecColour;
	texCoord = texCoordinate;
    gl_Position = vec4(vecPosition, 0.0,1.0);
}
