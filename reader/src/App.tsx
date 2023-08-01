"use-strict";

import { useState, useEffect, useRef } from "react";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import Top_Bar from "./components/top-bar";
import Bottom_Bar from "./components/bottom-bar";
import Reader from "./components/reader";
import "./App.css";

function App() {
  const [chapter, setChapter] = useState([["aw", "sharts"], ["no", "chapter"]]); 
  const [titles, setTitles] = useState(["aw sharts no titles"]);
  const [oui, setOui] = useState(0);

  async function load_chapter_titles() {
    setTitles(await invoke("get_chapter_titles"));
    await invoke("print_from_back", { message: "titles set" });
    await invoke("print_from_back", { message: titles.join(" ") });
  }

  async function load_chapter() {
    setOui(oui + 1);
    let arf: string[][] = await invoke("get_chapter", { title: ["Chapter", oui].join(" ") });
    setChapter(arf);
    console.log(oui);
  }

  let hello = useRef(null);

  useEffect(() => {
    console.log("hello:");
    hello?.current?.scrollIntoView({ behavior: 'smooth' });
  }, [oui]);


  return (
    <div>
      <div className="container">
        <div className="menu" onClick={() => { load_chapter(); }}></div>
          <div className="reader-wrapper">
            <Top_Bar index={oui}></Top_Bar>
            <Reader chapter={chapter} myRef={hello}></Reader>
            <Bottom_Bar></Bottom_Bar>
        </div>
      </div>
    </div>
  );
}

export default App;
