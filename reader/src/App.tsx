"use-strict";

import { useState, FC } from "react";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import Chapter_Block from "./components/chapter-block";
import Message from "./components/message"
import Reader from "./components/reader"
import "./App.css";

function App() {
  const [text, setText] = useState("");
  const [chapter, setChapter] = useState([["aw", "sharts"], ["no", "chapter"]]); 
  const [titles, setTitles] = useState(["aw sharts no titles"]);

  async function load_chapter_titles() {
    setTitles(await invoke("get_chapter_titles"));
    await invoke("print_from_back", { message: "titles set" });
    await invoke("print_from_back", { message: titles.join(" ") });
  }

  async function load_chapter() {
    let arf: string[][] = await invoke("get_chapter", { title: "Chapter 34" });
    setChapter(arf);
    console.log(chapter);
  }

  return (
    <div>
      <div className="container">
        <div className="menu" onClick={() => { load_chapter(); load_chapter_titles() }}></div>
          <div className="reader-wrapper">
            <div className="top-bar">
              <div className="to-name">To: </div>
              <div className="name">wassup</div>
            </div>
            <Reader chapter={chapter}></Reader>
            <div className="bot-bar">
              <div className="app-store"></div>
              <div className="search"></div>
              <div className="emoji"></div>
            </div>
        </div>
      </div>
    </div>
  );
}

export default App;
