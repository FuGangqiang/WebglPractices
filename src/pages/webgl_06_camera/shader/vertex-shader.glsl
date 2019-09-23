#version 300 es
precision mediump float;

uniform mat4 uModelMatrix;
uniform mat4 uViewMatrix;
uniform mat4 uProjectiveMatrix;

in vec3 aVertexPosition;
in vec4 aVertexColor;

out vec4 vVertexColor;

void main(void) {
    gl_Position = uProjectiveMatrix * uViewMatrix * uModelMatrix * vec4(aVertexPosition, 1.0);
    vVertexColor = aVertexColor;
}
