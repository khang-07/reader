import { useState } from "react";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [text, setText] = useState("");
  const [chapter, setChapter] = useState("");
  let i = 23;

  async function get_chapter(index: number) {
    setChapter(await invoke("abc", { index }));
    i++;
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
      <div className="menu" onClick={() => get_chapter(24)}>
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
