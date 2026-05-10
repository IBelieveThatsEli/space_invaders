#version 330 core

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoords;

out vec4 FragColor;

uniform vec3 lightDir;
uniform vec3 lightColor;
uniform vec3 ambientColor;
uniform vec3 baseColor;
uniform sampler2D ourTexture;
uniform int useTexture;

void main() {
    vec3 norm = normalize(Normal);
    vec3 lightDirection = normalize(-lightDir);

    float diff = max(dot(norm, lightDirection), 0.0);
    vec3 diffuse = diff * lightColor;
    vec3 ambient = ambientColor;

    vec3 albedo = baseColor;
    if (useTexture == 1) {
        albedo = texture(ourTexture, TexCoords).rgb;
    }

    vec3 result = (ambient + diffuse) * albedo;
    FragColor = vec4(result, 1.0);
}
