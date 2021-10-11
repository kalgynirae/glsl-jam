function main() {
  const canvas = document.getElementById("canvas");
  const gl = canvas.getContext("webgl");
  if (!gl) {
    alert("Failed to initialize WebGL");
    return;
  }

  const program = initShaders(gl);

  const posloc = gl.getAttribLocation(program, "aVertexPosition");
  const timeloc = gl.getUniformLocation(program, "uTime");
  const resloc = gl.getUniformLocation(program, "uResolution");

  const posbuf = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, posbuf);
  positions = [-1, 1, 1, 1, -1, -1, 1, -1];
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW);

  gl.clearColor(0, 0, 0, 1);
  gl.clear(gl.COLOR_BUFFER_BIT);

  const render = function(time) {
    gl.bindBuffer(gl.ARRAY_BUFFER, posbuf);
    gl.vertexAttribPointer(posloc, 2, gl.FLOAT, false, 0, 0);
    gl.enableVertexAttribArray(posloc);
    gl.useProgram(program);
    gl.uniform1f(timeloc, time / 1000);
    gl.uniform2f(resloc, canvas.clientWidth, canvas.clientHeight);
    gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
    requestAnimationFrame(render);
  }

  requestAnimationFrame(render);
}
window.onload = main;

function loadShader(gl, type, source) {
  const shader = gl.createShader(type);
  gl.shaderSource(shader, source);
  gl.compileShader(shader);
  if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
    alert(`Failed to compile shader: ${gl.getShaderInfoLog(shader)}`);
    gl.deleteShader(shader);
    return null;
  }
  return shader;
}

function initShaders(gl) {
  const vert = loadShader(
    gl,
    gl.VERTEX_SHADER,
    document.getElementById("vert").innerHTML
  );
  const frag = loadShader(
    gl,
    gl.FRAGMENT_SHADER,
    document.getElementById("frag").innerHTML
  );
  const program = gl.createProgram();
  gl.attachShader(program, vert);
  gl.attachShader(program, frag);
  gl.linkProgram(program);
  if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
    alert(`Failed to link shader program: ${gl.getProgramInfoLog(program)}`);
    return null;
  }
  return program;
}

function initBuffers(gl) {
  const posbuf = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, posbuf);
  positions = [-1, 1, 1, 1, -1, -1, 1, -1];
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW);
}
