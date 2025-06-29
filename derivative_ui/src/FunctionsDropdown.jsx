import React from 'react'

export default function FunctionsDropdown({addValue,function_map,setActiveMenu}) {

const addfuncValue=(value)=>{
    addValue(value);
    setActiveMenu("");
}
  return (
  <div>
    <section onClick={()=>setActiveMenu("")}  className='w-screen h-screen fixed bg-[rgba(150,150,150,0.4)]  top-0 left-0'></section >
            <section  className="w-64 border-2 border-black grid grid-cols-3 absolute left-2 sm:-left-4 bg-gray-200 dark:bg-[rgb(73,73,73)] rounded-sm p-1 gap-1 ">
             {function_map.map((func,i)=>{
              return <button onClick={()=>{addfuncValue(func)}}   className="rounded-md border-2 border-black p-2" key={i}>{func}</button>
             })}
            </section>
</div>
  )
}
