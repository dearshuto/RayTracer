import React, { useEffect, useRef, useState } from "react";
import ReactDOM from "react-dom/client";

import init, { InitOutput } from "runtime";


const RootComponent = () => {
    const [width, setWidth] = useState(256);
    const [height, setHeight] = useState(256);
    const [wasm, setWasm] = useState<InitOutput>();
    const canvasRef = useRef(null);

    useEffect(() => {
        console.log("useEffect()");
        init().then(wasm => {
            console.log("init()");
            setWasm(wasm);
        });
    }, []);

    return <>
        <p>Hello World</p>

        <label >Witdh
            <input type="number" value={width} onChange={e => setWidth(Number(e.target.value))}></input>
        </label>
        <label >Height
            <input type="number" value={height} onChange={e => setHeight(Number(e.target.value))}></input>
        </label>
        <button onClick={() => {
            if (canvasRef.current) {
                wasm?.render(canvasRef.current, width, height, 1/*sampling*/, 1);
            }
        }}>Run</button>
        <canvas id="canvas" ref={canvasRef} width={width} height={height}></canvas>
    </>;

}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <RootComponent />
    </React.StrictMode>,
);
