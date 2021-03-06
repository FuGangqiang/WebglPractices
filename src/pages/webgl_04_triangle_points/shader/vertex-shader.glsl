#version 300 es
precision mediump float;

in vec3 aVertexPosition;
in vec4 aVertexColor;

out vec4 vVertexColor;

void main(void) {
    gl_PointSize = 10.0;
    gl_Position = vec4(aVertexPosition, 1.0);
    vVertexColor = aVertexColor;
}
