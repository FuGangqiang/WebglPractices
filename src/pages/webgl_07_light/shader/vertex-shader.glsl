#version 300 es
precision mediump float;

uniform mat4 uModelMatrix;
uniform mat4 uViewMatrix;
uniform mat4 uProjectiveMatrix;
uniform mat4 uNormalMatrix;

uniform vec3 uLightDirection;
uniform vec4 uLightAmbient;
uniform vec4 uLightDiffuse;
uniform vec4 uMaterialAmbient;
uniform vec4 uMaterialDiffuse;

in vec3 aVertexPosition;
in vec3 aVertexNormal;

out vec4 vVertexColor;

void main(void) {
    vec4 transformedNormal =  uNormalMatrix * vec4(aVertexNormal, 1.0);
    float lambertTerm = dot(normalize(transformedNormal.xyz), normalize(uLightDirection));
    vVertexColor = uLightAmbient * uMaterialAmbient + uLightDiffuse * uMaterialDiffuse * lambertTerm;
    vVertexColor = vec4(vVertexColor.xyz, 1.0);

    gl_Position = uProjectiveMatrix * uViewMatrix * uModelMatrix * vec4(aVertexPosition, 1.0);
}
