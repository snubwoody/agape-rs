#version 410 

in vec2 position;
//in vec4 colour;
in vec2 uv;

uniform float width;
uniform float height;

out vec4 vertexColour;
out vec2 texCoord;


float normalizeValue(float value, float minValue, float maxValue){
    if (value > maxValue){
		value = maxValue;
	}
	
    if (value < minValue) {
		value = minValue;
	}

	float scale =  (1.0 - -1.0) / (maxValue - minValue);
	float offset = minValue*(scale) + -1;
	

	return value * scale + offset;
}

void main() {
	vertexColour = vec4(1.0,1.0,1.0,1.0);
	texCoord = uv;
	float xPos = normalizeValue(position.x,0,width);
	float yPos = -normalizeValue(position.y,0,height);
    gl_Position = vec4(xPos,yPos, 0.0,1.0);
}



