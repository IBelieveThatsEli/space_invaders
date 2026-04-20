#version 330 core

int vec2 TexCoords;

out vec4 FragColor;

uniform sampler2D ourTexture;

void main() {
    FragColor = /*texture(ourTexture, TexCoords)*/ vec4(1.0, 0.0, 0.0, 1.0);
}
