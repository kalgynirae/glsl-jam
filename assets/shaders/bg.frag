#version 450
// vim: ft=c

layout(location = 0) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform TimeUniform_value {
    float time;
};
layout(set = 3, binding = 0) uniform WindowsizeUniform_value {
    vec2 windowsize;
};

vec2 tileCoords() {
    return (gl_FragCoord.xy - windowsize * 0.5) / 64. * (cos(time*0.5) * 0.15 + 1);
}

#define PI 3.141592653589793
#define TWO_PI 6.283185307179586

vec3 lch_to_lab(vec3 lch) {
    float L = lch.x * 100;
    float a = lch.y * 100 * cos(lch.z * TWO_PI);
    float b = lch.y * 100 * sin(lch.z * TWO_PI);
    return vec3(L, a, b);
}

vec3 lab_to_rgb(vec3 lab) {
    float L = lab.x;
    float a = lab.y;
    float b = lab.z;

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
    vec2 coords = tileCoords();
    vec2 tile = floor(coords);
    float iteration = floor((time+tile.x+tile.y)*0.5);

    float huebase = floor(random(tile+0.1*iteration) * 10.) / 10.;
    float hue = huebase + sin(time*.5) * .3;

    float edgeness = pow(1 - abs(sin(coords.x * PI)), 2) + pow(1 - abs(sin(coords.y * PI)), 2);
    float lightness = 0.4 - edgeness * 0.05;

    vec3 color = lab_to_rgb(lch_to_lab(vec3(lightness, .4, hue)));

    float alpha = 1.;

    o_Target = vec4(color, alpha);
}
