import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import '@radix-ui/themes/styles.css';
import { Flex, Text, Button, Box } from '@radix-ui/themes';
import './assets/wasm_execute.js';
import wasmUrl from './assets/index.wasm?url';
import wasm from "vite-plugin-wasm";
// import init from './assets/index.wasm';

// import * as wasmModulePath from './assets/index.wasm';

async function loadWasmModule(url) {
  try {
    // Fetch the wasm file
    const response = await fetch(url, { headers: { 'Content-Type': 'application/wasm' } });

    // Check if the fetch was successful
    if (!response.ok) {
      throw new Error(`Failed to fetch wasm module: ${response.statusText}`);
    }

    // Convert the fetched data into a WebAssembly module
    const wasmArrayBuffer = await response.arrayBuffer();
    const wasmModule = await WebAssembly.instantiate(wasmArrayBuffer);

    // The instance of the module is now ready to be used
    console.log('WASM module loaded successfully');
    return wasmModule.instance.exports;
  } catch (error) {
    console.error('Error loading the WASM module:', error);
    return null;
  }
}



async function load_wasm(url) {
  const go = new Go();
  let mod, inst;
  await WebAssembly.instantiateStreaming(fetch(url), go.importObject).then(async (result) => {
    mod = result.module;
    inst = result.instance;
    go.run(inst);
    inst = await WebAssembly.instantiate(mod, go.importObject);
    console.log("inst is ", inst)
  });

  // console.log("ddddd inst %o", inst);
  // let data = wasmGenerateCron("18 20 * * * ");
  // console.log("data is %o", data)


}

// init().then((instance) => {
//   instance.exports.test()
// })


function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>
      <SomeColumns></SomeColumns>
      <Flex gap="3">
        <Box width="64px" height="64px">
          <input />
        </Box>
        <Box width="64px" height="64px">
          <div></div>
        </Box>
      </Flex>
    </div>
  );
}


function SomeColumns() {
  return (
    <Flex direction="column" gap="2">
      <Text>Hello from Radix Themes :)</Text>
      <Button>Let's go</Button>
      <GoCronParser></GoCronParser>
    </Flex>
  );
}


const GoCronParser = () => {



  const [input, setInput] = useState('');
  const [output, setOutput] = useState('');

  const handleInputChange = async (event: any) => {
    const value = event.target.value;
    console.log("input data is %s", value)
    setInput(value);
    await load_wasm(wasmUrl);
    let got = wasmGenerateCron(value);
    setOutput(got);
    console.log("finish")
    // const wasmGenerateCron = wasmModulePath.wasmGenerateCron;
    // const got = wasmGenerateCron(value);
    // console.log("got ", got)

  };

  return (
    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', padding: '20px' }}>
      <input
        type="text"
        value={input}
        onChange={handleInputChange}
        placeholder="Enter text to parse"
        style={{ flex: 1, marginRight: '20px' }}
      />
      <div style={{ flex: 1, border: '1px solid gray', padding: '10px', minHeight: '100px' }}>
        {output}
      </div>
    </div>
  );
};

export default App;
