// #version 330 core

// in vec3 FragPos;
// in vec3 Normal;
// in vec2 TexCoords;

// out vec4 FragColor;

// uniform vec3 lightDir;
// uniform vec3 lightColor;
// uniform vec3 ambientColor;
// uniform vec3 baseColor;
// uniform sampler2D ourTexture;
// uniform int useTexture;

// void main() {
//     vec3 norm = normalize(Normal);
//     vec3 lightDirection = normalize(-lightDir);

//     float diff = max(dot(norm, lightDirection), 0.0);
//     vec3 diffuse = diff * lightColor;
//     vec3 ambient = ambientColor;

//     vec3 albedo = baseColor;
//     if (useTexture == 1) {
//         albedo = texture(ourTexture, TexCoords).rgb;
//     }

//     vec3 result = (ambient + diffuse) * albedo;
//     FragColor = vec4(result, 1.0);
// }
#version 330 core

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoords;

out vec4 FragColor;

uniform vec3 lightDir;
uniform vec3 lightColor;
uniform vec3 ambientColor;

uniform vec4 baseColorFactor;
uniform vec3 emissiveFactor;
uniform float metallicFactor;
uniform float roughnessFactor;

uniform sampler2D ourTexture;
uniform int useTexture;
uniform int alphaMode;
uniform int doubleSided;

void main() {
    vec3 norm = normalize(Normal);

    if (doubleSided == 1 && !gl_FrontFacing) {
        norm = -norm;
    }

    vec3 lightDirection = normalize(-lightDir);
    float diff = max(dot(norm, lightDirection), 0.0);

    vec3 ambient = ambientColor;
    vec3 diffuse = diff * lightColor;

    vec4 albedo = baseColorFactor;
    if (useTexture == 1) {
        albedo *= texture(ourTexture, TexCoords);
    }

    float roughness = clamp(roughnessFactor, 0.04, 1.0);
    float metallic = clamp(metallicFactor, 0.0, 1.0);

    vec3 lit = (ambient + diffuse * (1.0 - 0.5 * roughness)) * albedo.rgb;
    vec3 emissive = emissiveFactor;
    vec3 finalColor = mix(lit, lit * 0.5, metallic) + emissive;

    float alpha = albedo.a;

    if (alphaMode == 1 && alpha < 0.5) {
        discard;
    }

    FragColor = vec4(finalColor, alpha);
}
