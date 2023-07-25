"use-strict";

import { useState } from "react";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import Message from "./components/message";
import "./App.css";
import Message_Block from "./components/message-block";

function App() {
  const [text, setText] = useState("");
  const [chapter, setChapter] = useState({}); 
  const [titles, setTitles] = useState([""]);

  async function load_chapter_titles() {
    setTitles(await invoke("get_chapter_title"));
    await invoke("print_from_back", { message: "titles set" });
    await invoke("print_from_back", { message: titles.join(" ") });
  }

  async function load_chapter() {
    let arf = await invoke("get_chapter", { message: "The Boy Named Crow" });
  }

  return (
    <div>
      <div className="container">
        <div className="menu" onClick={() => load_chapter()}>
        </div>
        <div className="curtain"></div>
          <div className="reader">
            <div className="test">
              <Message_Block content={["hi", "ho", "ha", "he", "hu"]} type="send"></Message_Block>
              <Message_Block content={["ki", "ko", "ka", "ke", "ku"]} type="receive"></Message_Block>
              <Message_Block content={titles} type="receive"></Message_Block>
            </div>
        </div>
      </div>
    </div>
  );
}

export default App;
