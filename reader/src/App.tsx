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
  
  async function get_path() {
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
      hello
    </div>
  );
}

export default App;
