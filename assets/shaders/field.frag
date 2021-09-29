#version 450
layout(location = 0) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;
layout(set = 2, binding = 0) uniform FieldShaderSettings_time {
    float time;
};

layout(set = 3, binding = 0) uniform FieldShaderSettings_speed {
    float speed;
};


bool IsGridLine(vec2 fragCoord, float time, float speed) {
    // this affects the # of lines
    float base = 0.05;
    float x_modifier = 30.0; // this is the same as SCALE in field.rs
	vec2 vPixelsPerGridSquare = vec2(base / x_modifier, base / x_modifier);
	
	vec2 vScreenPixelCoordinate = fragCoord.xy;
    vScreenPixelCoordinate.x += time * speed; 
	vec2 vGridSquareCoords = fract(vScreenPixelCoordinate / vPixelsPerGridSquare);
	
	vec2 vGridSquarePixelCoords = vGridSquareCoords * vPixelsPerGridSquare;

    // this is like.. line width
	vec2 vIsGridLine = step(vGridSquarePixelCoords, vec2((base / x_modifier) / 10, (base / x_modifier) / 10));
	
	float fIsGridLine = max(vIsGridLine.x, vIsGridLine.y);

	return fIsGridLine > 0.5;
}

void main() {
	vec3 vResult = vec3(0.0);

    if (IsGridLine(v_Uv.xy, time, speed)) {
        vResult.b = 255;
        vResult.r = 240;
    }

    o_Target = vec4(vResult, 1.0);
}
