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
    await invoke("print_from_back", { message: "hi from front" });
  }

  return (
    <div>
      <div className="container">
        <div className="menu">

        </div>
        <div className="curtain"></div>
        <div className="test">
          <div className="reader">
            <p className="send">Are you Are you Are you Are you Are you Are you Are you Are you Are you Are you Are you Are you Are you Are you Are you Are you</p>
            <p className="receive">I am I am I am I am I am I am I am I am I am I am I am I am I am I am I am I am I am I am I am I am I am I am I am I am</p>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
