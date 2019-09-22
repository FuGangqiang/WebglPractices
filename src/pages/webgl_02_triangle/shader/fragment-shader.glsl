#version 300 es
precision mediump float;

in vec4 vVertexColor;

out vec4 fragColor;

void main(void) {
    fragColor = vVertexColor;
}
