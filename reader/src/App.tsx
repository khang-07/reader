"use-strict";

import { useState } from "react";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [text, setText] = useState("");
  const [chapter, setChapter] = useState("");

  async function load_book() {
    await invoke("beep");
    console.log("hi");
  }
  
  async function show_Text() {
    try {
      const path = await open({
        multiple: false,
        directory: false,
        title: "Open File"
      });

      setText(path?.toString()!);

    } catch (err) {
      console.log(err);
    }
  }

  return (
    <div className="container">
      <div className="menu" onClick={() => load_book()}>
      </div>
      <div className="curtain">
        
      </div>
      <div className="reader">
        {chapter}
      </div>
    </div>
  );
}

export default App;
