#include <metal_stdlib>

using namespace metal;

vertex float4 vertexShader(
    uint vertexID [[vertex_id]],
    device packed_float3 *vertices [[buffer(0)]]
) {
    float4 vertexOutPositions = float4(vertices[vertexID], 1.0);
    return vertexOutPositions;
}

fragment float4 fragmentShader(float4 vertexOutPositions [[stage_in]]) {
    return float4(182.0f/255.0f, 240.0f/255.0f, 228.0f/255.0f, 1.0f);
}
