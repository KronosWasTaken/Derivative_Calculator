import React, { useState, useEffect,useRef } from 'react';
import Lottie from 'lottie-react';
import Dark from "./assets/Darkmode.json"
export default function Darkmode() {
 const [Darkmode,setDark]=useState(()=>{
    let initial=localStorage.getItem("dark");
    return initial?initial:false;
  });
  const LottieRef=useRef();
  useEffect(()=>{
    const handleanim=()=>{
   localStorage.setItem("dark",Darkmode);

LottieRef.current.setSpeed(8);

if(localStorage.getItem("dark")=="true"){
LottieRef.current.playSegments([0,200],true);
document.documentElement.classList.add("dark");
}
else{
LottieRef.current.playSegments([200,400],true);
document.documentElement.classList.remove("dark");

}

 }
 handleanim();

 },[Darkmode])

  return (
    <button className='w-20 absolute right-4 top-7 ' onClick={() => setDark(prev => !prev)}>
      <Lottie animationData={Dark} loop={0} autoplay={false} lottieRef={LottieRef} ></Lottie>
    </button>
  );
}
