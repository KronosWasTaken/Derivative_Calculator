import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import FunctionsDropdown from "./FunctionsDropdown";
import Manual from "./manual";
import Darkmode from './Darkmode';


function App() {
const inputRef = useRef(null); 
const variableRef=useRef(null);

const [variableName,setVariableName]=useState("x");


  const [inputext, setinputext] = useState("");
  const [outputtext, setoutputtext] = useState("");
  const [history, setHistory] = useState([]);



  const [active_menu,setActiveMenu]=useState("");

  const calc_state=["log","7","8","9","/","exp","4","5","6","*","sqrt","1","2","3","-","abs","0",".","+"];

  const trig_functions=["sin","cos","tan","cosec","sec","cot"];

  const inverse_functions=["arcsin","arccos","arctan","arccosec","arcsec","arccot"];

  const hyperbolic_functions=["sinh","cosh","tanh","cosehc","sech","coth","arsinh","arcosh","artanh","arcosech","arsech","arcoth"];

  const constant_vals=["pi","e","inf","deg"];

  const simple_functions=["log","exp","sqrt","abs"]

const all_functions = [
  ...trig_functions,
  ...inverse_functions,
  ...hyperbolic_functions,
  ...simple_functions
];






useEffect(() => {
  const handleKeyDown = async (e) => {
    if(!variableRef.current){return;}
if (variableRef.current === document.activeElement) {
      return;
    }


    const el = inputRef.current;
    if (!el) return;
    el.focus();
      }

   


  window.addEventListener("keydown", handleKeyDown);

  return () => window.removeEventListener("keydown", handleKeyDown);
}, [inputext]);


  async function getDerivative() {
    try {
      const response = await invoke("find_der", {
        inputExpr: inputext,
        diffVar: variableName.trim()||"x",
      });
      setoutputtext(response);
      return response;
    } catch (error) {
      console.error("Error calling find_der:", error);
      return "Error";
    }
  }




const addValue = async (value) => {
  const el = inputRef.current;
  if (!el) return;

  const start = el.selectionStart;
  const end = el.selectionEnd;

  if (value === "clear") {
    setinputext("");
    setoutputtext("");
    return;
  }

  if (value === "cross") {
    
    if (start === end && start > 0) {
      setinputext((prev) => prev.slice(0, start - 1) + prev.slice(end));
      setTimeout(() => {
        el.selectionStart = el.selectionEnd = start - 1;
        el.focus();
      }, 0);
    }
    return;
  }

  if (value === "=") {
    const response = await getDerivative();
    setHistory((prev) => [...prev, { input: inputext, output: response }]);
    setinputext("");
    return;
  }

  let textToInsert = value;


  if (value === "^" || all_functions.includes(value)) {
    textToInsert += "(";
  }

 
  setinputext((prev) => 
    prev.slice(0, start) + textToInsert + prev.slice(end)
  );


  setTimeout(() => {
    el.selectionStart = el.selectionEnd = start + textToInsert.length;
    el.focus();
  }, 0);
};



  return (
    <main className="flex w-screen h-screen overflow-x-hidden dark:bg-[rgb(65,65,65)] dark:text-gray-400  history">
  
     
      <div className="w-full">
        <section className="m-4 border-b-2 p-4 rounded-xl  shadow-lg history text-3xl border-black">
          <div className="flex relative flex-col mb-2">

            <input className="h-20 w-auto p-4 outline-none" value={inputext} ref={inputRef}
onChange={(e) => {
  setinputext(e.target.value);


}}

onKeyDown={(e)=>{
  if(e.key=="^"){
     e.preventDefault(); 
    addValue("^");
  }
  if(e.key=="Enter" || e.key=="="){
      e.preventDefault(); 
    addValue("=");
  }
}}
type="text" placeholder="Enter expression"/>

<Darkmode/>

<input
  type="text"
  className="h-20 w-auto p-4 outline-none"
  value={variableName}
  onKeyDown={(e) => {
    const key = e.key;
    if (/^[a-z]$/.test(key)) {
      e.preventDefault();
      setVariableName(key);
    } else if (
      key === 'Backspace' ||
      key === 'Delete' ||
      key === 'Tab' ||
      key === 'ArrowLeft' ||
      key === 'ArrowRight'
    ) {
      // allow control/navigation keys
    } else {
      e.preventDefault(); // block everything else
    }
  }}
  ref={variableRef}
  placeholder="Enter Variable"
/>

         
          </div>
          <div className="flex flex-col">
            <input
              className="h-20 w-auto p-4 outline-none"
              type="text"
              value={outputtext}
              placeholder="Result"
              readOnly
            />
          </div>
        </section>

        <section className="px-8 flex gap-2 sm:gap-5 text-sm">

             <div className="  sm:relative">
            <button onClick={()=>setActiveMenu("trig")}>Trignometry</button>
{active_menu=="trig" && <FunctionsDropdown addValue={addValue} setActiveMenu={setActiveMenu}  function_map={trig_functions}/>}
</div>

          <div className="  sm:relative">
            <button onClick={()=>setActiveMenu("hyper")}>Hyperbolic</button>
{active_menu=="hyper" && <FunctionsDropdown addValue={addValue} setActiveMenu={setActiveMenu}  function_map={hyperbolic_functions}/>}
</div>

<div className="sm:relative">
<button onClick={()=>setActiveMenu("inverse")}>Inverse</button>
{active_menu=="inverse" && <FunctionsDropdown addValue={addValue} setActiveMenu={setActiveMenu}  function_map={inverse_functions}/>}
  
</div>

<div className="sm:relative">
<button onClick={()=>setActiveMenu("constants")}>constants</button>
{active_menu=="constants" && <FunctionsDropdown addValue={addValue} setActiveMenu={setActiveMenu}  function_map={constant_vals}/>}
</div>
  



<div className="sm:relative">
<button onClick={()=>setActiveMenu("manual")}>Manual</button>
{active_menu=="manual" && <Manual setActiveMenu={setActiveMenu}/>
}
  
</div>


        </section>



        <section className="text-xl m-4 grid grid-cols-5 gap-[0.2rem] ">


 <button
          
              className="border-2 border-black rounded-md shadow-lg  w-auto p-2 md:px-4  transition-all"
              onClick={() => addValue("(")}
            >(</button>

             <button
          
              className="border-2 border-black rounded-md shadow-lg  w-auto p-2 md:px-4  transition-all"
              onClick={() => addValue(")")}
            >)</button>


      <button
          
              className="border-2 border-black rounded-md shadow-lg  w-auto p-2 md:px-4  transition-all"
              onClick={() => addValue("^")}
            >

           ^
            </button>




      <button className="border-2 border-black rounded-md shadow-lg  w-auto p-2 md:px-4  transition-all" onClick={() => addValue("clear")}>

           CE
            </button>


            <button className="border-2 border-black rounded-md shadow-lg  w-auto flex justify-center items-center  transition-all" onClick={() => addValue("cross")} >
             
             <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px" fill="black"><path d="m456-320 104-104 104 104 56-56-104-104 104-104-56-56-104 104-104-104-56 56 104 104-104 104 56 56Zm-96 160q-19 0-36-8.5T296-192L80-480l216-288q11-15 28-23.5t36-8.5h440q33 0 56.5 23.5T880-720v480q0 33-23.5 56.5T800-160H360ZM180-480l180 240h440v-480H360L180-480Zm400 0Z"/></svg>
          
             
            </button>
     
          {calc_state.map((symbol, i) => (
            <button key={i}className="border-2 border-black rounded-md shadow-lg w-auto p-2 md:py-4  transition-all" onClick={() => addValue(symbol)}>
              {symbol}
            </button>
          ))}




           <button className="border-2 border-black rounded-md shadow-lg w-auto p-2 md:py-4  transition-all" onClick={() => addValue("=")} >
              =
            </button>
        </section>
      </div>

<div id="history" className="p-8 hidden md:block  text-2xl  w-auto h-screen ">
        <h1 className="mb-4 ">History</h1>
      <div className="w-96 h-96">
        <div className="flex flex-col h-[80vh] gap-1 overflow-y-scroll">
          {history.map(({ input, output }, i) => (
           <div className="flex flex-col pb-2">
            <p className="text-sm opacity-75">{input}</p>
            <p>{output}</p>
           </div>
          ))}
        </div>
      </div>
      </div>
    </main>
  );
}

export default App;
