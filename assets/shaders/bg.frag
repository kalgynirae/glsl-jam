#version 450
// vim: ft=c

layout(location = 0) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform TimeUniform_value {
    float time;
};

vec2 tileCoords() {
    return (gl_FragCoord.xy - 240.) / 64.;
}

vec2 screenCoords() {
    //vec2 RESOLUTION = vec2(480);
    //return (gl_FragCoord.xy - 0.5 * RESOLUTION) / min(RESOLUTION.x, RESOLUTION.y);
    return (gl_FragCoord.xy - 240.) / 480.;
}

#define TWO_PI 6.283185307179586

vec3 lch(float L_in, float C_in, float H) {
    float L = L_in * 100;
    float C = C_in * 100;

    // LCH to Lab
    float a = C * cos(H * TWO_PI);
    float b = C * sin(H * TWO_PI);

    // Lab to XYZ
    float fy = (L + 16.) / 116.;
    float fx = a / 500. + fy;
    float fz = fy - b / 200.;

    float junction_e = 0.008856;
    float junction_k = 903.3;

    float fx3 = pow(fx, 3.);
    float test = step(junction_e, fx3);
    float xr = test * fx3 + (1.-test) * ((116. * fx - 16.) / junction_k);

    test = step(junction_e * junction_k, L);
    float yr = test * pow(((L + 16.) / 116.), 3.) + (1.-test) * (L / junction_k);

    float fz3 = pow(fz, 3.);
    test = step(junction_e, fz3);
    float zr = test * fz3 + (1.-test) * ((116. * fz - 16.) / junction_k);

    float X = xr * 0.9504;
    float Y = yr * 1.0;
    float Z = zr * 1.0888;

    //mat3 xyz_to_srgb = mat3( 3.2404542, -1.5371385, -0.4985314,
    //                        -0.9692660,  1.8760108,  0.0415560,
    //                         0.0556434, -0.2040259,  1.0572252);
    mat3 xyz_to_srgb = mat3( 3.2404542, -0.9692660,  0.0556434,
                            -1.5371385,  1.8760108, -0.2040259,
                            -0.4985314,  0.0415560,  1.0572252);
    // XYZ to sRGB
    return xyz_to_srgb * vec3(X, Y, Z);
}

float random(vec2 st) {
    return fract(sin(dot(st.xy, vec2(12.9898,78.233)))*43758.5453123);
}

void main() {
    vec2 coords = screenCoords();
    float distanceFromCenter = length(coords);
    float mask = 1. - smoothstep(0.4, 0.403, distanceFromCenter) * 0.6;

    float speed = 1.5;
    float translation = sin(time * speed);
    float percentage = 0.6;
    float threshold = v_Uv.x + translation * percentage;

    vec3 red = lch(.4, .8, .08);
    vec3 blue = lch(.3, .8, .8);
    vec3 mixed = mix(red, blue, threshold);

    vec3 color = mixed * mask;
    float alpha = random(floor(tileCoords()));
    o_Target = vec4(color, alpha);
}
