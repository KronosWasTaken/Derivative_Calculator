import React from "react";

export default function Manual({setActiveMenu}) {
  return (
    <div>
  <section onClick={()=>setActiveMenu("")} className='w-screen h-screen fixed bg-[rgba(150,150,150,0.8)] top-0 left-0'></section>

  <div 
    className="p-4 border-2 dark:dark:bg-[rgb(65,65,65)] bg-white fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 max-w-[90vw] max-h-[90vh] overflow-auto"
  >
    <button  onClick={()=>setActiveMenu("")} className="fixed right-4 top-2 text-3xl">x</button>
    <h2 className="text-2xl font-bold mb-4">Derivative Calculator Manual</h2>

    <p className="mb-2">You can input expressions using the following syntax:</p>
    <ul className="list-disc list-inside mb-4">
      <li><code>sin(x)</code> : Sine of x</li>
      <li><code>cos(x)</code> : Cosine of x</li>
      <li><code>tan(x)</code> : Tangent of x</li>
      <li><code>exp(x)</code> : Exponential function, e<sup>x</sup></li>
      <li><code>log(x)</code> : Natural logarithm (ln(x))</li>
      <li><code>sin^2(x)</code> : (sin(x))²</li>
      <li><code>cos^3 x</code> : (cos(x))³</li>
      <li><code>2sinx</code> : 2 × sin(x)</li>
      <li><code>x^3 + 2x + 1</code> : Polynomial</li>
      <li><code>(x+1)*(x-1)</code> : Parentheses for grouping</li>
      <li><code>pi</code> : Mathematical constant π (3.14159…)</li>
      <li><code>e</code> : Euler's number (2.71828…)</li>
      <li><code>deg</code> : Degree conversion constant (π/180)</li>
      <li> For multiple equations in a function like sin2x or such use sin(2x)  </li>
            <li> The above rule applies for power in functions eg use sin^(2x) dont use sin^2x   </li>
            
    </ul>

    <h3 className="text-xl font-semibold mb-2">Tips:</h3>
    <ul className="list-disc list-inside space-y-1">
      <li>You can use implicit multiplication: <code>2x</code> means 2×x, <code>sin2x</code> means sin(2×x)</li>
      <li>You can use powers on functions: <code>sin^2(x)</code> means (sin(x))²</li>
      <li>You can use parentheses for clarity: <code>sin^2(x+1)</code></li>
      <li>Supported functions: sin, cos, tan, exp, log, sinh, cosh, tanh, etc.</li>
      <li>Supported constants: pi, π, e, euler, deg, degree</li>
    </ul>
  </div>
</div>

  );
}
